use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{error::Error, fmt::Debug};
use tracing::{debug, trace};

/// Represents a move that a player can make.
#[derive(Debug, TryFromPrimitive, PartialEq, IntoPrimitive, Clone)]
#[repr(u8)]
pub enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

impl Eq for RPSChoice {}

impl RPSChoice {
    // assumption: all characters are ascii
    pub const MY_CHAR_BASE: u8 = b'A';
    pub const OPPONENT_CHAR_BASE: u8 = b'X';

    /// Converts an encoded Rock Paper Scissors move
    /// into the RPSChoice type used in the rest of the program
    #[tracing::instrument]
    pub fn from_char(value: char, char_base: u8) -> Self {
        trace!(
            "subtracting {char_base} ({}) from {} ({value})",
            char_base as char,
            value as u8
        );
        // Converting the letter into a number lets us do math on it.
        // We can subtract the value of the "char_base" (either `A` or `X`),
        // so that A/X are 0, B/Y are 1, and C/Z are 2.
        let choice = ((value as u8) - char_base)
            // These numbers line up with the numeric representation of the enum,
            // so we can just cast it to RPSChoice.
            .try_into()
            .unwrap_or_else(|_| panic!("{value} is not a valid RPS move"));
        debug!(
            "{value} is a {choice:?} choice (char base: {})",
            char_base as char
        );
        choice
    }

    /// Returns the choice that is superior to this one
    /// (e.g. `Scissors.get_losing_move() == Rock`)
    #[tracing::instrument]
    pub fn get_losing_move(&self) -> RPSChoice {
        let number_representation: u8 = self.clone().into();
        // the next item in the enum will always beat this one
        let losing_move = (number_representation + 1)
            .try_into()
            // if this is the last item, we just loop back to the start
            .unwrap_or_else(|_| 0_u8.try_into().unwrap());
        trace!("{self:?} loses to {losing_move:?}");
        losing_move
    }

    /// Returns the points you wil automatically get for making this choice,
    /// even if you don't win
    ///
    /// Rock gets 1, Paper gets 2 and Scissors gets 3
    #[tracing::instrument]
    pub fn get_points(&self) -> u8 {
        let number_representation: u8 = self.clone().into();
        // Our enum is already ordered correctly so we can just
        // add 1 to the 0-indexed numeric representation
        let points_earned = number_representation + 1;
        trace!("{self:?} choice earns {points_earned} points");
        points_earned
    }
}

/// Represents the outcome of a round of Rock Paper Scissors.
#[derive(Debug, IntoPrimitive, Clone)]
#[repr(u8)]
pub enum RPSResult {
    Loss,
    Draw,
    Win,
}

impl RPSResult {
    /// Returns the points you will earn for achieving this outcome
    ///
    /// Loss gets 0, Draw gets 3 and Win gets 6
    #[tracing::instrument]
    pub fn get_points(&self) -> u8 {
        let number_representation: u8 = self.clone().into();

        // Again, our enum is already ordered correctly.
        // However, it's counting in steps of 3 rather than 1
        // so we have to multiply everything by 3.
        // Note that we are not adding 1 here because the
        // intended scores are 0-indexed as well.
        let points_earned = number_representation * 3;
        trace!("{self:?} earns {points_earned} points");
        points_earned
    }
}

impl From<(&RPSChoice, &RPSChoice)> for RPSResult {
    /// Returns whether the player on the left side wins, loses, or is at a draw
    #[tracing::instrument(name = "result_from_choices")]
    fn from(choices: (&RPSChoice, &RPSChoice)) -> Self {
        if choices.0 == choices.1 {
            trace!("{:?} is a draw", choices.0);
            RPSResult::Draw
        } else if &choices.0.get_losing_move() == choices.1 {
            trace!("{:?} loses to {:?}", choices.0, choices.1);
            RPSResult::Loss
        } else {
            trace!("{:?} wins to {:?}", choices.0, choices.1);
            RPSResult::Win
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn solve(lines: impl Iterator<Item = Result<String, impl Error>> + Debug) -> u32 {
    lines
        .map(|line| match line {
            Ok(line) => line,
            Err(err) => panic!("error while reading input: {err}"),
        })
        // There might be a trailing newline, which should be ignored.
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(' ')
                // converting String to char simplifies the solving process
                .map(|instruction| instruction.chars().next().expect("invalid format in input"))
                .collect::<Vec<char>>()
        })
        .fold(0, |total_points, moves| {
            let mut moves = moves.into_iter();
            let my_move = moves.next().expect("invalid format in input");
            let opponent_move = moves.next().expect("invalid format in input");
            trace!("my move code: {my_move:?}; opponent move code: {opponent_move:?}");

            let my_move = RPSChoice::from_char(my_move, RPSChoice::MY_CHAR_BASE);
            let opponent_move = RPSChoice::from_char(opponent_move, RPSChoice::OPPONENT_CHAR_BASE);
            let play_result: RPSResult = (&my_move, &opponent_move).into();
            debug!(
                "my move: {my_move:?}; opponent move: {opponent_move:?}; result: {play_result:?}"
            );

            // Both functions return a u8 type.
            // We need to cast that to a u32 in order to add it to the toal.
            let points_gained: u32 = (my_move.get_points() + play_result.get_points()).into();
            let new_total = total_points + points_gained;
            debug!("i earn {points_gained} points ({total_points} -> {new_total})");
            new_total
        })
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
