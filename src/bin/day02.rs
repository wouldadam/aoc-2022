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

enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(input: &str) -> Result<Outcome, Self::Err> {
        match input {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

impl Outcome {
    fn needed_choice(&self, first_move: Choice) -> Choice {
        match *self {
            Outcome::Lose => match first_move {
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper,
            },
            Outcome::Draw => first_move,
            Outcome::Win => match first_move {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock,
            },
        }
    }
}

fn main() {
    let input = include_str!("../../assets/day02.txt");

    // Parse the file into scores for each game
    let a_scores = input.split('\n').map(|game| {
        let moves = game.split(' ').collect::<Vec<_>>();
        let opp_move = Choice::from_str(moves[0]).unwrap();
        let my_move = Choice::from_str(moves[1]).unwrap();

        let result_score = my_move.scores(opp_move);
        let choice_score: i32 = my_move as i32;

        choice_score + result_score
    });

    // Sum part a scores
    let a_total: i32 = a_scores.sum();
    println!("Part a total score: {}", a_total);

    // Parse the file into scores for each game
    let b_scores = input.split('\n').map(|game| {
        let moves = game.split(' ').collect::<Vec<_>>();
        let opp_move = Choice::from_str(moves[0]).unwrap();
        let outcome = Outcome::from_str(moves[1]).unwrap();

        let my_move = outcome.needed_choice(opp_move);

        let outcome_score = outcome as i32;
        let choice_score: i32 = my_move as i32;

        choice_score + outcome_score
    });

    // Sum part b scores
    let b_total: i32 = b_scores.sum();
    println!("Part b total score: {}", b_total);
}
