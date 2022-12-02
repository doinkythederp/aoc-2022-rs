use std::{error::Error, fmt::Debug, iter::Iterator};
use tracing::{debug, info, trace};

/// Input: Inventories, seperated by blank line
///
/// Output: Highest calorie count of any inventory
#[tracing::instrument]
pub fn solve(lines: impl Iterator<Item = Result<String, impl Error>> + Debug) -> usize {
    debug!("Iterating over input lines to find the biggest calorie count");

    let mut biggest_inventory_so_far = 0_usize;
    let mut elf_num = 1_usize;
    let mut current_elf_inventory = 0_usize;

    for (line_number, line) in lines.enumerate() {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    if current_elf_inventory > biggest_inventory_so_far {
                        debug!("elf #{elf_num} has the biggest inventory so far @ {current_elf_inventory} calories");
                        biggest_inventory_so_far = current_elf_inventory;
                    }
                    current_elf_inventory = 0;
                    elf_num += 1;
                    continue;
                }

                let item_calorie_count: usize = line.parse().unwrap_or_else(|err| {
                    panic!("invaild format in input (line {line_number}): {err} ({err:?})")
                });
                current_elf_inventory += item_calorie_count;

                trace!("new calorie count for elf #{elf_num} is {current_elf_inventory}");
            }
            Err(err) => panic!("error while reading input: {err} ({err:?})"),
        }
    }

    // commit inventory size in case there's no trailing newline in the input
    if current_elf_inventory > biggest_inventory_so_far {
        info!("elf #{elf_num} has the biggest inventory so far @ {current_elf_inventory} calories");
        biggest_inventory_so_far = current_elf_inventory;
    }

    biggest_inventory_so_far
}

#[cfg(test)]
mod tests {
    use tracing::info;

    use super::super::get_lines;
    use std::time::Instant;

    use super::solve;

    #[test]
    fn d1p2() {
        let input_lines = get_lines();

        let before_solve = Instant::now();
        let solution = solve(input_lines);
        let total_time = before_solve.elapsed().as_millis();

        info!("SOLUTION: {solution} calories ({total_time}ms)");
    }
}
