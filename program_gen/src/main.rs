use std::collections::HashSet;
use std::io;

use clap::Parser;
use fuzz_harness::harness::{LehmerRng, NUM_INPUT_BYTES, TOTAL_COVERAGE_CODE_BLOCKS};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The maximum number of bits to use
    #[clap(long, default_value_t = 1024)]
    max_bits: usize,

    /// The minimum size of the program
    #[clap(long, default_value_t = 4000)]
    min_size: u64,

    /// The maximum number of failures
    #[clap(long, default_value_t = 1)]
    max_failures: usize,
}

macro_rules! record_covered_code_block {
    ($code:expr, $block_count:expr, $depth:expr) => {{
        $code.push_str(&format!(
            "{}if covered_code_blocks[{}] == 0 {{ new_block_reached = true; }}\n",
            "    ".repeat($depth),
            $block_count
        ));
        $code.push_str(&format!(
            "{}covered_code_blocks[{}] = 1;\n",
            "    ".repeat($depth),
            $block_count
        ));
        $block_count += 1;
    }};
}

macro_rules! close_brackets {
    ($code:expr, $depth:expr) => {{
        while $depth > 1 {
            $depth -= 1;
            $code.push_str(&format!("{}}}\n", "    ".repeat($depth)));
        }
    }};
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut rng = LehmerRng::with_seed(42);
    let mut output_code = String::new();
    let mut used_indices: HashSet<usize> = HashSet::new();
    let end_chance = NUM_INPUT_BYTES;

    let mut depth = 1;
    let mut block_count = 0u64;
    let mut failed_attempts = 0;

    output_code += format!("#[inline(never)] pub fn fuzzable_function(input: &[u8; {}], covered_code_blocks: &mut [u64; {}]) -> bool {{\n", NUM_INPUT_BYTES, TOTAL_COVERAGE_CODE_BLOCKS).as_str();
    output_code += "    let mut new_block_reached = false;\n";

    record_covered_code_block!(output_code, block_count, depth);

    while block_count < args.min_size || rng.next() % end_chance != 0 {
        if rng.next() % 4 == 0 {
            if let Some((start, end)) = find_bits(&mut rng, &mut used_indices, args.max_bits) {
                let byte_index = start / 8;
                let mask = create_mask(start % 8, end % 8);

                output_code += &format!(
                    "    {}if input[{}] & {:#010b} == {:#010b} {{\n",
                    "    ".repeat(depth),
                    byte_index,
                    mask,
                    rng.next() as u8 & mask
                );

                depth += 1;
                record_covered_code_block!(output_code, block_count, depth);
            } else {
                failed_attempts += 1;
                if failed_attempts >= args.max_failures {
                    break;
                }
            }
        }

        if depth > 1 && rng.next() % 4 == 0 {
            depth -= 1;
            output_code += &format!("{}}}\n", "    ".repeat(depth));
        }
    }

    close_brackets!(output_code, depth);
    output_code += &format!("    {}\n}}\n", "new_block_reached");

    write_and_compile_program(output_code)
}

fn find_bits(
    rng: &mut LehmerRng,
    used: &mut HashSet<usize>,
    max_bits: usize,
) -> Option<(usize, usize)> {
    let mut attempts = 0;

    while attempts < 2000 {
        let start = rng.next() % max_bits;
        let end = (start + rng.next() % 8).min(max_bits - 1);

        if (start / 8 == end / 8) && (start..=end).all(|bit| !used.contains(&bit)) {
            for bit in start..=end {
                used.insert(bit);
            }
            return Some((start, end));
        }

        attempts += 1;
    }

    None
}

fn create_mask(start_bit: usize, end_bit: usize) -> u8 {
    let mask_start = 0xFF >> start_bit;
    let mask_end = 0xFF << (7 - end_bit);
    mask_start & mask_end
}

fn write_and_compile_program(code: String) -> io::Result<()> {
    std::fs::write("./fuzz_harness/src/generated_fuzz_target.rs", &code)?;
    Ok(())
}
