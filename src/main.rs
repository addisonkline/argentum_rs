mod card;
mod ranker;

use card::Card;
use ranker::score_hand;

fn main() {
    let hand = vec![
        Card::new("2".to_string(), "h".to_string()),
        Card::new("7".to_string(), "d".to_string()),
    ];

    // Community cards
    let table = vec![
        Card::new("9".to_string(), "c".to_string()),
        Card::new("J".to_string(), "s".to_string()),
        Card::new("4".to_string(), "h".to_string()),
        Card::new("K".to_string(), "h".to_string()),
        Card::new("6".to_string(), "d".to_string()),
    ];

    let result = score_hand(hand, table);
    println!("{}", result)
}
