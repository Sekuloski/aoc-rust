use std::{char::MAX, u32};

use::aoc_helper::{AocDay, Puzzle};

fn part_one(input: String) -> i32 {
    let mut answer: i32 = 0;
    for text in input.split("\n") {
        let mut first_number: u32 = MAX as u32;
        let mut last_number: u32 = MAX as u32;
        for number in text.chars() {
            if number.is_numeric() {
                if first_number == MAX as u32 {
                    first_number = match number.to_string().parse::<u32>() {
                        Ok(num) => { num },
                        Err(_) => { last_number },
                    };
                }
                last_number = match number.to_string().parse::<u32>() {
                    Ok(num) => { num },
                    Err(_) => { last_number },
                };
            }
        }
        let digit = format!("{first_number}{last_number}");
        answer += match digit.parse::<i32>() {
            Ok(number) => { number },
            Err(_) => { 
                println!("{digit}");
                0
            },
        };
    }

    return answer;
}

fn part_two(input: String) -> i32 {
    let mut answer: i32 = 0;
    return answer;
}

fn main() {
    let session_id = "53616c7465645f5f4cec89389726bcf5fb012489a03c03e0973e371c2fa1d6383ddd2c31a62803c54384ea06cc091aabf2f3f1042abb5926c8ff40920a593407";
    let mut day_1 = AocDay::new(2023, 2).with_session_id(session_id);
    let test_input = 
"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    let part_1 = Puzzle::new(
        1,
        |x: String| part_one(x)
    )
    .with_examples(&[test_input]);

    let part_2 = Puzzle::new(
        2,
        |x: String| part_two(x)
    )
    .with_examples(&[test_input]);

    match day_1.run(&part_1) {
        Ok(_) => {},
        Err(error) => { println!("{error}")},
    };
    match day_1.run(&part_2) {
        Ok(_) => {},
        Err(error) => { println!("{error}")},
    };
}

