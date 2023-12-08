mod card;
use std::fs;

use crate::hand::Hand;

mod hand;

fn read_lines(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Couldn't read input");
    let lines = content
        .lines()
        .map(|s| -> String { s.trim().to_string() })
        .collect();

    lines
}

fn compute_winnings(hands: &mut [Hand]) -> u32 {
    hands.sort();

    let total_winnings = hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx as u32 + 1) * hand.bid)
        .sum();

    total_winnings
}

fn main() {
    let lines = read_lines("res/data.txt");
    // let lines = read_lines("res/data_light.txt");
    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        let hand = Hand::from(line.as_str());
        let hand_type = hand.get_type();
        println!("{:?} {:?}", hand_type, hand);
        hands.push(hand);
    }

    let winnings = compute_winnings(&mut hands);
    println!("Winnings: {}", winnings);
}
