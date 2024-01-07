use std::io::Write;

pub const NUM_INPUT_BYTES: usize = 128;
pub const TOTAL_COVERAGE_CODE_BLOCKS: usize = 560;
pub const MAX_CORES: usize = 2000;
const LEHMER_A: usize = 48271;
const LEHMER_M: usize = 0x7fffffff;

// Random Number Generator struct
pub struct LehmerRng {
    current_state: usize,
}

impl LehmerRng {
    pub fn with_seed(seed: usize) -> LehmerRng {
        LehmerRng {
            current_state: seed,
        }
    }

    pub fn next(&mut self) -> usize {
        self.current_state = (self.current_state.wrapping_mul(LEHMER_A)) % LEHMER_M;
        self.current_state
    }
}

#[derive(Debug)]
pub enum Outcome {
    ReachedFullCoverage {
        duration_seconds: f64,
    },
    TimedOut {
        coverage_achieved: f64,
        duration_seconds: f64,
    },
}

pub struct Config<W: Write> {
    /// If enabled, the fuzzer will use a single DB of coverage results between all cores.
    pub share_coverage_results: bool,
    /// If enabled, the fuzzer will use a single DB of fuzz inputs between all cores.
    pub share_inputs_database: bool,
    /// Number of cores to simulate in a fuzzer run.
    pub num_cores_to_simulate: usize,
    /// The time in seconds the fuzzer is allowed to run.
    pub timeout_seconds: Option<f64>,
    /// The percent of the input that we can mutate in each fuzz case. For example,
    /// if set to 5, the mutator will mutate up to 5% of the input bytes in
    /// a given fuzz case.
    pub mutation_aggressiveness_percent: usize,
    pub coverage_per_fuzz_case_writer: Option<W>,
}

impl<W: Write> Default for Config<W> {
    fn default() -> Self {
        Self {
            share_coverage_results: true,
            share_inputs_database: true,
            num_cores_to_simulate: MAX_CORES,
            timeout_seconds: None,
            mutation_aggressiveness_percent: 5,
            coverage_per_fuzz_case_writer: None,
        }
    }
}

pub struct FuzzTester<W: Write> {
    pub cases_count: u64,
    inputs: Vec<Vec<[u8; NUM_INPUT_BYTES]>>,
    coverage: Vec<[u64; TOTAL_COVERAGE_CODE_BLOCKS]>,
    cfg: Config<W>,
    rng: LehmerRng,
    max_bytes_to_mutate: usize,
}

impl<W: Write> FuzzTester<W> {
    pub fn new(seed: usize, cfg: Config<W>) -> Self {
        let mut inputs_db = Vec::with_capacity(MAX_CORES);
        let mut coverage_db = Vec::with_capacity(MAX_CORES);
        for _ in 0..MAX_CORES {
            inputs_db.push(Vec::new());
            coverage_db.push([0; TOTAL_COVERAGE_CODE_BLOCKS]);
        }
        let max_bytes_to_mutate = ((NUM_INPUT_BYTES as f64)
            * cfg.mutation_aggressiveness_percent as f64
            / 100f64) as usize;
        Self {
            cases_count: 0,
            inputs: inputs_db,
            coverage: coverage_db,
            cfg,
            rng: LehmerRng::with_seed(seed),
            max_bytes_to_mutate: max_bytes_to_mutate.max(1),
        }
    }

    pub fn update_cfg(&mut self, cfg: Config<W>) {
        let max_bytes_to_mutate = ((NUM_INPUT_BYTES as f64)
            * cfg.mutation_aggressiveness_percent as f64
            / 100f64) as usize;
        self.max_bytes_to_mutate = max_bytes_to_mutate.max(1);
        self.cfg = cfg;
    }

    #[inline]
    fn clear_databases(&mut self) {
        let input_db_size = if self.cfg.share_inputs_database {
            1
        } else {
            self.cfg.num_cores_to_simulate
        };
        let coverage_db_size = if self.cfg.share_coverage_results {
            1
        } else {
            self.cfg.num_cores_to_simulate
        };

        self.inputs
            .iter_mut()
            .take(input_db_size)
            .for_each(|db| db.clear());
        self.coverage
            .iter_mut()
            .take(coverage_db_size)
            .for_each(|db| db.fill(0));
    }

    #[inline]
    fn randomize_input(
        max_bytes_to_mutate: usize,
        rng: &mut LehmerRng,
        input: &mut [u8; NUM_INPUT_BYTES],
    ) {
        // Randomly choose how many bytes to mutate, up to the maximum
        let num_bytes_to_mutate = rng.next() % max_bytes_to_mutate + 1;
        for _ in 0..num_bytes_to_mutate {
            input[rng.next() % NUM_INPUT_BYTES] = rng.next() as u8;
        }
    }

    #[inline]
    fn compute_coverage_percentage(coverage_db: &mut [u64; TOTAL_COVERAGE_CODE_BLOCKS]) -> f64 {
        (coverage_db.iter().filter(|&&x| x > 0).count() as f64 / TOTAL_COVERAGE_CODE_BLOCKS as f64)
            * 100.0
    }

    pub fn execute<F>(&mut self, mut test_fn: F) -> Outcome
    where
        F: FnMut(&[u8; NUM_INPUT_BYTES], &mut [u64; TOTAL_COVERAGE_CODE_BLOCKS]) -> bool,
    {
        self.clear_databases();

        let mut fuzz_input = [0; NUM_INPUT_BYTES];
        let mut total_cases = 0u64;
        let mut rng = &mut self.rng;

        let input_db_size = if self.cfg.share_inputs_database {
            1
        } else {
            self.cfg.num_cores_to_simulate
        };
        let coverage_db_size = if self.cfg.share_coverage_results {
            1
        } else {
            self.cfg.num_cores_to_simulate
        };
        let mutation_aggressiveness = self.cfg.mutation_aggressiveness_percent;
        let max_bytes_to_mutate = self.max_bytes_to_mutate;

        loop {
            for core in 0..self.cfg.num_cores_to_simulate {
                total_cases += 1;

                let input_index = core % input_db_size;
                let coverage_index = core % coverage_db_size;
                let input_db = &mut self.inputs[input_index];
                let coverage_db = &mut self.coverage[coverage_index];

                if !input_db.is_empty() {
                    fuzz_input.copy_from_slice(&input_db[rng.next() % input_db.len()]);
                }

                Self::randomize_input(max_bytes_to_mutate, &mut rng, &mut fuzz_input);

                if test_fn(&fuzz_input, coverage_db) {
                    input_db.push(fuzz_input);
                    let cov_percent = Self::compute_coverage_percentage(coverage_db);
                    if let Some(w) = self.cfg.coverage_per_fuzz_case_writer.as_mut() {
                        write!(
                            w,
                            "{}, {}, {}\n",
                            total_cases, cov_percent, mutation_aggressiveness
                        )
                        .unwrap();
                    }
                    if cov_percent == 100.0 {
                        return Outcome::ReachedFullCoverage {
                            duration_seconds: total_cases as f64
                                / self.cfg.num_cores_to_simulate as f64,
                        };
                    }
                }

                if let Some(max) = self.cfg.timeout_seconds {
                    if total_cases as f64 / self.cfg.num_cores_to_simulate as f64 >= max {
                        return Outcome::TimedOut {
                            coverage_achieved: Self::compute_coverage_percentage(coverage_db),
                            duration_seconds: total_cases as f64
                                / self.cfg.num_cores_to_simulate as f64,
                        };
                    }
                }
            }
        }
    }
}
