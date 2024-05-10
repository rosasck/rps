/**
 * @file types.rs
 * @brief Type Declarations
 * @author Katie Stoltz
 *
 */

// RPSMove: Available Moves

pub enum RPSMove {
    Rock,
    Paper,
    Scissors,
}

// RPSJudge: Judging Options
pub enum RPSJudge {
    RockBeatsScissors,
    PaperBeatsRock,
    ScissorsBeatsPaper,
}

// RPSResult: Results for plays
pub enum RPSResult {
    Win(RPSJudge),
    Loss(RPSJudge),
    Tie(String),
}

// RPSMoveError: Unknown moves
#[derive(Debug)]
pub enum RPSMoveError {
    Unknown(String),
}
