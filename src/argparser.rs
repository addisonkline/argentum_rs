use clap::{Arg, Parser};
use crate::card::Card;

pub enum ArgType {
    SimulateBoth,
    SimulateNoBoard,
    SimulateNoP2,
    SimulateNeither
}

/// Program to calculate hand odds in heads-up Texas Hold'em
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Player 1's hand
    #[arg(long)]
    p1: String,

    /// Player 2's hand (optional)
    #[arg(long, default_value = "[]")]
    p2: String,

    /// Table cards (optional)
    #[arg(short, long, default_value = "[]")]
    table: String,

    /// Number of simulations to run
    #[arg(short, long, default_value_t = 10000)]
    num: u32,
}

pub fn convert_args_to_tuple(args: &Args) -> (ArgType, Vec<Card>, Vec<Card>, Vec<Card>, u32) {
    // create vectors to populate
    let h1 = string_to_card_vec(&args.p1);
    let h2 = string_to_card_vec(&args.p2);
    let table = string_to_card_vec(&args.table);
    let arg_type: ArgType;
    let mut n = args.num;

    if h2.is_empty() && table.is_empty() {
        arg_type = ArgType::SimulateNeither;
    }
    else if h2.is_empty() {
        arg_type = ArgType::SimulateNoP2;
    }
    else if table.is_empty() {
        arg_type = ArgType::SimulateNoBoard;
    }
    else { // all information is provided, so only 1 simulation will be done
        arg_type = ArgType::SimulateBoth;
        if n != 1 {
            println!("[Ag] note: with both p2 and table cards given, no simulations are necessary. the same player wins every time");
        }
        n = 1;
    }

    return (arg_type, h1, h2, table, n);
}

fn string_to_card_vec(input: &String) -> Vec<Card> {
    // Remove the square brackets from the input string
    let s = input.trim_matches(|c| c == '[' || c == ']');
    // if the string passed in is "[]", then return an empty vector
    if s.is_empty() {
        return Vec::new();
    }
    // Split the string by commas to get individual card strings
    let card_strings = s.split(',');
    let mut cards = Vec::new();

    // Parse each card string into a Card struct
    for card_str in card_strings {
        let card_str_trimmed = card_str.trim();
        let card = parse_card(card_str_trimmed);
        cards.push(card);
    }

    return cards;
}

fn parse_card(s: &str) -> Card {
    // Extract the value notation and suit notation from the card string
    let value_n = &s[..s.len()-1];
    let suit_n  = &s[s.len()-1..];

    // Map the value notation to the card's value name and numeric value
    let (value, value_numeric) = match value_n {
        "2" => ("2", 2),
        "3" => ("3", 3),
        "4" => ("4", 4),
        "5" => ("5", 5),
        "6" => ("6", 6),
        "7" => ("7", 7),
        "8" => ("8", 8),
        "9" => ("9", 9),
        "T" => ("10", 10),
        "J" => ("J", 11),
        "Q" => ("Q", 12),
        "K" => ("K", 13),
        "A" => ("A", 14),
        _   => ("?", 0), // Handle invalid value notations
    };

    // Map the suit notation to the suit name
    let suit = match suit_n {
        "s" => "s",
        "h" => "h",
        "d" => "d",
        "c" => "c",
        _   => "?", // Handle invalid suit notations
    };

    let identity = s.to_string();

    return Card {
        value: value.to_string(),
        suit: suit.to_string(),
        value_numeric,
        identity,
    };
}