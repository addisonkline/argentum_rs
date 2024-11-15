mod card;
mod ranker;
mod simulate;

use crate::card::Card;
use crate::ranker::score_hand;
use crate::simulate::simulate_no_p2;

fn main() {
    let hand = vec![
        Card::new("A".to_string(), "h".to_string()),
        Card::new("A".to_string(), "d".to_string()),
    ];

    // Community cards
    let table = vec![
        Card::new("9".to_string(), "c".to_string()),
        Card::new("J".to_string(), "s".to_string()),
        Card::new("4".to_string(), "h".to_string()),
        Card::new("K".to_string(), "h".to_string()),
        Card::new("6".to_string(), "d".to_string()),
    ];

    let result = score_hand(hand.clone(), table.clone());
    println!("Hand score for p1: {}", result);

    let (w1, w2) = simulate_no_p2(hand.clone(), table.clone(), 10000);
    println!("Player 1 wins {} times out of 10000", w1);
    println!("Player 2 wins {} times out of 10000", w2);
    println!("There was a tie {} times out of 10000", 10000 as u32 - w1 - w2);
}
