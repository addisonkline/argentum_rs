use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(PartialEq, Clone, Debug)]
pub struct Card {
    pub value: String,
    pub suit: String,
    pub value_numeric: u8,
    pub identity: String,
}

impl Card {
    pub fn new(value: String, suit: String) -> Self {
        Self { value: value.clone(), suit: suit.clone(), value_numeric: Self::get_value_numeric(&value), identity: Self::get_identity(value, suit.clone()) }
    }

    fn get_value_numeric(value: &str) -> u8 {
        let number: u8 = match value {
            "A"=> 14,
            "K" => 13,
            "Q" => 12,
            "J" => 11,
            "T" => 10,
            "9" => 9,
            "8" => 8,
            "7" => 7,
            "6" => 6,
            "5" => 5,
            "4" => 4,
            "3" => 3,
            "2" => 2,
            _ => 0
        };
        return number;
    }

    fn get_identity(value: String, suit: String) -> String {
        let identity: String = value + &suit;
        return identity;
    }
}

/// Generate a deck of 52 cards
pub fn generate_deck() -> Vec<Card> {
    let values: Vec<&str> = vec!["2","3","4","5","6","7","8","9","T","J","Q","K","A"];
    let suits: Vec<&str> = vec!["s","h","c","d"];

    let mut deck: Vec<Card> = vec![];

    for value in &values {
        for suit in &suits {
            let this_card = Card::new(value.to_string(), suit.to_string());
            deck.push(this_card);
        }
    }
    return deck;
}

/// Shuffle deck
pub fn shuffle_deck(deck: &mut Vec<Card>) {
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);
}