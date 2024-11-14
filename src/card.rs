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

