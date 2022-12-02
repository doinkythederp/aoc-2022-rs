use std::{error::Error, fmt::Debug, iter::Iterator};
use tracing::{debug, trace};

/// Always sorted from biggest to smallest
#[derive(Debug)]
pub struct Leaderboard([u32; Leaderboard::LEADERBOARD_SIZE]);

impl Leaderboard {
    pub const LEADERBOARD_SIZE: usize = 3;

    #[tracing::instrument]
    pub fn process_contestant(&mut self, new_contestant: u32) {
        for (standing_contestant_index, standing_contestant) in self.0.iter().enumerate() {
            if new_contestant > *standing_contestant {
                trace!(
                    "new contestant with value {new_contestant} has taken position #{} (old: {standing_contestant})",
                    standing_contestant_index + 1
                );

                for ranking_index in
                    ((standing_contestant_index + 1)..Leaderboard::LEADERBOARD_SIZE).rev()
                {
                    let new_value = self.0[ranking_index - 1];
                    trace!(
                        "#{} now has value {new_value} (was {})",
                        ranking_index + 1,
                        self.0[ranking_index]
                    );
                    self.0[ranking_index] = new_value;
                }
                self.0[standing_contestant_index] = new_contestant;

                debug!("leaderboard has updated: {self:?}");
                break;
            }
        }
    }

    pub fn get_top_values(self) -> [u32; Leaderboard::LEADERBOARD_SIZE] {
        self.0
    }
}

#[tracing::instrument]
pub fn solve(lines: impl Iterator<Item = Result<String, impl Error>> + Debug) -> u32 {
    debug!("Iterating over input lines to populate the leaderboard");

    let mut leaderboard = Leaderboard([0, 0, 0]);
    let mut elf_num = 1_u32;
    let mut current_elf_inventory = 0_u32;

    for (line_number, line) in lines.enumerate() {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    trace!("finished collecting inventory for elf #{elf_num}");
                    leaderboard.process_contestant(current_elf_inventory);
                    current_elf_inventory = 0;
                    elf_num += 1;
                    continue;
                }

                let item_calorie_count: u32 = line.parse().unwrap_or_else(|err| {
                    panic!("invaild format in input (line {line_number}): {err} ({err:?})")
                });
                current_elf_inventory += item_calorie_count;

                trace!("new calorie count for elf #{elf_num} is {current_elf_inventory}");
            }
            Err(err) => panic!("error while reading input: {err} ({err:?})"),
        }
    }

    // commit inventory size in case there's no trailing newline in the input
    if current_elf_inventory != 0 {
        leaderboard.process_contestant(current_elf_inventory);
    }

    leaderboard.get_top_values().into_iter().sum()
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
