use::aoc_helper::{AocDay, Puzzle};

fn part_one(input: String) -> i32 {
    let mut answer: i32 = 0;
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

