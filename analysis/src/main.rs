use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    io::Write,
    sync::{atomic::AtomicU64, Mutex},
};

use fuzz_harness::{
    generated_fuzz_target::{self},
    harness::{Config, FuzzTester, Outcome, MAX_CORES},
};

const RNG_SEED: usize = 42;

// Analyzes different fuzzer configurations at differ numbers of cores, with configurable
// timeouts. The goal is to figure out which fuzzer configurations reach the highest amount
// of coverage at different numbers of cores and different timeouts.
//
// It also runs an experiment to understand the impact of linear core scaling on
// the time to reach full coverage of a fuzzer.
//
// Finally, it runs an experiment to understand the impact of mutator strategy
// aggressiveness on coverage percentage reached with the best fuzzer configuration.
fn main() {
    let mut f = File::create("notebooks/coverage_reached_with_timeout.csv").unwrap();
    write!(
        f,
        "num_cores,coverage_amount_reached,duration_seconds,share_coverage_results,share_inputs_database\n",
    )
    .unwrap();
    let timeouts = [0.1, 0.5, 1.0, 10.0];
    for timeout in timeouts {
        perform_fuzzer_analysis(&mut f, Some(timeout), generate_full_task_suite);
    }

    // Next, performs an analysis of how long it would take to reach full coverage for
    // the best fuzzer configuration at different numbers of cores.
    let mut f = File::create("notebooks/times_to_full_coverage.csv").unwrap();
    write!(
        f,
        "num_cores,coverage_amount_reached,duration_seconds,share_coverage_results,share_inputs_database\n",
    )
    .unwrap();
    // Run the fuzzer analysis with no timeout.
    perform_fuzzer_analysis(&mut f, None, || -> HashSet<AnalysisTask> {
        let mut tasks = HashSet::new();
        for x in 1..=NUM_CORES_DATA_GRANULARITY {
            let num_cores_to_simulate = compute_num_cores_to_simulate(x);
            tasks.insert(AnalysisTask {
                share_coverage_results: true,
                share_inputs_database: true,
                num_cores_to_simulate,
                mutation_aggressiveness_percent: 5, // Mutate up to 5% of the input bytes in each fuzz case.
            });
        }
        tasks
    });

    // Next performs an experiment where we try out different mutation aggressiveness
    // strategies vs. number of fuzz cases to observe the amount of coverage reached.
    // We will perform this experiment with a single simulated core.
    let fname = "notebooks/mutation_aggressiveness.csv".to_string();
    let mut f = File::create(&fname).unwrap();
    write!(f, "fuzz_cases,coverage_reached,mutation_aggressiveness\n",).unwrap();

    let mutation_aggressiveness_percents = [5, 10, 25, 50];
    for pct in mutation_aggressiveness_percents {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(&fname)
            .unwrap();
        let mut func_to_fuzz = generated_fuzz_target::fuzzable_function;
        let mut fuzzer = FuzzTester::new(
            RNG_SEED,
            Config {
                share_coverage_results: true,
                share_inputs_database: true,
                num_cores_to_simulate: 1,
                mutation_aggressiveness_percent: pct,
                timeout_seconds: None,
                coverage_per_fuzz_case_writer: Some(f),
            },
        );
        let Outcome::ReachedFullCoverage { duration_seconds } = fuzzer.execute(&mut func_to_fuzz)
        else {
            panic!("Unexpected outcome");
        };
        println!(
            "Num fuzz cases to reach full coverage {}, aggressiveness percent {}",
            duration_seconds, pct,
        );
    }
}

const NUM_CORES_DATA_GRANULARITY: usize = 100;
const NUM_THREADS: usize = 10;

#[derive(PartialEq, Eq, Hash)]
struct AnalysisTask {
    share_inputs_database: bool,
    share_coverage_results: bool,
    num_cores_to_simulate: usize,
    mutation_aggressiveness_percent: usize,
}

struct AnalysisResult {
    share_inputs_database: bool,
    share_coverage_results: bool,
    num_cores_to_simulate: usize,
    coverage_amount_reached: f64,
    duration_seconds: f64,
}

fn generate_full_task_suite() -> HashSet<AnalysisTask> {
    let mut tasks = HashSet::new();
    for &share_inputs_database in &[false, true] {
        for &share_coverage_results in &[true, false] {
            for x in 1..=NUM_CORES_DATA_GRANULARITY {
                let num_cores_to_simulate = compute_num_cores_to_simulate(x);
                tasks.insert(AnalysisTask {
                    share_coverage_results,
                    share_inputs_database,
                    num_cores_to_simulate,
                    mutation_aggressiveness_percent: 5, // (Default) mutate up to 5% of the input bytes in each fuzz case.
                });
            }
        }
    }
    tasks
}

fn compute_num_cores_to_simulate(x: usize) -> usize {
    (x as f64 / NUM_CORES_DATA_GRANULARITY as f64 * MAX_CORES as f64) as usize
}

fn perform_fuzzer_analysis<F>(file: &mut File, time_constraint: Option<f64>, task_generator: F)
where
    F: Fn() -> HashSet<AnalysisTask>,
{
    let tasks = Mutex::new(task_generator().into_iter().collect::<Vec<AnalysisTask>>());
    let results = Mutex::new(Vec::new());
    let total_tasks = AtomicU64::new(0);

    std::thread::scope(|s| {
        for _ in 0..NUM_THREADS {
            s.spawn(|| {
                let mut fuzzer: FuzzTester<File> = FuzzTester::new(RNG_SEED, Config::default());
                let mut f = generated_fuzz_target::fuzzable_function;
                loop {
                    let AnalysisTask {
                        share_inputs_database,
                        share_coverage_results,
                        num_cores_to_simulate,
                        mutation_aggressiveness_percent,
                    } = match tasks.lock().unwrap().pop() {
                        Some(task) => task,
                        None => return,
                    };
                    fuzzer.update_cfg(Config {
                        share_inputs_database,
                        share_coverage_results,
                        num_cores_to_simulate,
                        timeout_seconds: time_constraint,
                        mutation_aggressiveness_percent,
                        coverage_per_fuzz_case_writer: None,
                    });

                    // Track if any of the tests found all possible coverage
                    // during a time constrained mode. This will indicate that
                    // the data is invalid and should not be used.
                    let mut fuzz_run_duration_seconds = 0f64;

                    // Average a bunch of fuzzer executions for smoother results.
                    // If no time constraint is specified, it means we want to run the fuzzer
                    // until we find full coverage, so one single run is needed.
                    let total_data_points: usize = match time_constraint {
                        Some(_) => 500,
                        None => 1,
                    };
                    let mut coverage_achieved_sum = 0f64;
                    for _ in 0..total_data_points {
                        match fuzzer.execute(&mut f) {
                            Outcome::ReachedFullCoverage { duration_seconds } => {
                                fuzz_run_duration_seconds = duration_seconds;
                                coverage_achieved_sum += 100.0; // 100% coverage reached.
                                break;
                            }
                            Outcome::TimedOut {
                                coverage_achieved,
                                duration_seconds,
                            } => {
                                coverage_achieved_sum += coverage_achieved;
                                fuzz_run_duration_seconds = duration_seconds;
                            }
                        };
                    }
                    let avg_cov_achieved = coverage_achieved_sum / total_data_points as f64;
                    total_tasks.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    println!(
                        "mean cov percent achieved {}, total tasks completed {}",
                        avg_cov_achieved,
                        total_tasks.load(std::sync::atomic::Ordering::SeqCst)
                    );

                    // Record the results
                    results.lock().unwrap().push(AnalysisResult {
                        num_cores_to_simulate,
                        share_inputs_database,
                        share_coverage_results,
                        coverage_amount_reached: avg_cov_achieved,
                        duration_seconds: fuzz_run_duration_seconds,
                    });
                }
            });
        }
    });
    log_results(file, results)
}

fn log_results(file: &mut File, results: Mutex<Vec<AnalysisResult>>) {
    let mut results = results.lock().unwrap();
    results.sort_by_key(|x| x.num_cores_to_simulate);
    for r in results.iter() {
        let AnalysisResult {
            num_cores_to_simulate,
            coverage_amount_reached,
            duration_seconds,
            share_coverage_results,
            share_inputs_database,
        } = r;
        write!(
            file,
            "{}, {}, {}, {}, {}\n",
            num_cores_to_simulate,
            coverage_amount_reached,
            duration_seconds,
            share_coverage_results,
            share_inputs_database,
        )
        .unwrap();
    }
}
