use std::{error::Error, fmt::Debug};

/// Represents a move that a player can make.
#[derive(Debug, PartialEq)]
enum RPSMove {
    Rock,
    Paper,
    Scissors,
}

impl RPSMove {
    fn get_inferior_move(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn play_against(&self, other: &Self) -> RPSOutcome {
        if self == other {
            return RPSOutcome::Draw;
        }

        if &self.get_inferior_move() == other {
            return RPSOutcome::Win;
        }

        RPSOutcome::Loss
    }
}

enum RPSOutcome {
    Win,
    Loss,
    Draw,
}

impl From<char> for RPSMove {
    /// Converts an encoded Rock Paper Scissors move
    /// into the RPSChoice type used in the rest of the program
    fn from(value: char) -> Self {
        match value {
            'a' | 'x' => Self::Rock,
            'b' | 'y' => Self::Paper,
            'c' | 'z' => Self::Scissors,
            _ => panic!("{value} is not a valid RPS move"),
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn solve(lines: impl Iterator<Item = Result<String, impl Error>> + Debug) -> u32 {
    let _ = lines
        // Simple conversion to something we can use.
        // In this case that means a String.
        .map(|line| match line {
            Ok(line) => line,
            Err(err) => panic!("error while reading puzzle input: {err}"),
        })
        // remove the trailing newline if present
        .filter(|line| !line.is_empty())
        // parse the lines into moves
        .map(|line| {
            // Lines are in the following format: `A X`.
            // The first letter is our move, the 2nd is the opponent's.
            // Those are the only parts we care about, so we get indexes 0 and 2.

            let my_move = RPSMove::from(line.chars().next().unwrap());
            let opponent_move = RPSMove::from(line.chars().nth(2).unwrap());
            (my_move, opponent_move)
        });

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::solve;
    use crate::get_lines;
    use std::time::Instant;
    use tracing::info;

    #[test]
    fn d2p1() {
        let input_lines = get_lines("day_2.txt");

        let before_solve = Instant::now();
        let solution = solve(input_lines);
        let total_time = before_solve.elapsed();

        info!("SOLUTION: {solution} points ({total_time:?})");
    }
}
