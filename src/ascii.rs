#[derive(PartialEq, Copy, Clone)]
pub enum Hand {
    Scissors,
    Rock,
    Paper,
    None,
}

pub fn str_to_enum(str: &str) -> Hand {
    match str {
        "scissors" => {
            Hand::Scissors
        }

        "rock" => {
            Hand::Rock
        }

        "paper" => {
            Hand::Paper
        }

        _ => Hand::None
    }
}

pub fn enum_to_str<'a>(hand: Hand) -> &'a str {
    match hand {
        Hand::None => "",
        Hand::Scissors => "scissors",
        Hand::Rock => "rock",
        Hand::Paper => "paper",
    }
}

pub fn print_ascii(z: &Hand, rev: bool) {
    match z {
        Hand::Scissors => output(include_str!("../ascii/scissors"), rev),
        Hand::Rock => output(include_str!("../ascii/rock"), rev),
        Hand::Paper => output(include_str!("../ascii/paper"), rev),
        Hand::None => {}
    };
}

fn output(line: &str, rev: bool) {
    if !rev {
        println!("{}", line);
    } else {
        println!("{}", line.chars().rev().collect::<String>());
    }
}