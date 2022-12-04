/// Rock paper scissors, playing games as in strategy guide input data. Each line is a game.
/// Score 1/2/3 for playing RPS respectively. Score 0/3/6 for loss/draw/win respectively.
/// ABC = RPS from opponent. XYZ = your move.
/// Print total score.
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Choice {
    fn scores(&self, opp_move: Choice) -> i32 {
        match *self {
            Choice::Rock => match opp_move {
                Choice::Rock => 3,
                Choice::Paper => 0,
                Choice::Scissors => 6,
            },
            Choice::Paper => match opp_move {
                Choice::Rock => 6,
                Choice::Paper => 3,
                Choice::Scissors => 0,
            },
            Choice::Scissors => match opp_move {
                Choice::Rock => 0,
                Choice::Paper => 6,
                Choice::Scissors => 3,
            },
        }
    }
}

impl FromStr for Choice {
    type Err = ();

    fn from_str(input: &str) -> Result<Choice, Self::Err> {
        match input {
            "X" | "A" => Ok(Choice::Rock),
            "Y" | "B" => Ok(Choice::Paper),
            "Z" | "C" => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}

fn main() {
    let input = include_str!("../../assets/day2a.txt");

    // Parse the file into scores for each game
    let game_scores = input.split('\n').map(|game| {
        let moves = game.split(' ').collect::<Vec<_>>();
        let opp_move = Choice::from_str(moves[0]).unwrap();
        let my_move = Choice::from_str(moves[1]).unwrap();

        let result_score = my_move.scores(opp_move);
        let choice_score: i32 = my_move as i32;

        choice_score + result_score
    });

    // Sum scores
    let total: i32 = game_scores.sum();

    // 12156
    println!("Total score: {}", total);
}
