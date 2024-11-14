use crate::Card;
use std::collections::HashMap;
use itertools::Itertools;

fn sort_cards_by_value_desc(cards: &mut Vec<&Card>) {
    cards.sort_by(|a, b| b.value_numeric.cmp(&a.value_numeric));
}

fn has_straight_flush(hand: &Vec<&Card>) -> bool {
    return has_straight(hand) && has_flush(hand);
}

fn has_quads(hand: &Vec<&Card>) -> bool {
    let mut value_counts = HashMap::new();

    for card in hand {
        *value_counts.entry(&card.value).or_insert(0) += 1;
    }

    return value_counts.values().any(|&count| count == 4);
}

fn has_full_house(hand: &Vec<&Card>) -> bool {
    let mut value_counts = HashMap::new();

    for card in hand {
        *value_counts.entry(&card.value).or_insert(0) += 1;
    }

    return (value_counts.values().any(|&count| count == 2)) && (value_counts.values().any(|&count| count == 3));
}

fn has_flush(hand: &Vec<&Card>) -> bool {
    let mut suit_counts = HashMap::new();

    for card in hand {
        *suit_counts.entry(&card.suit).or_insert(0) += 1;
    }

    return suit_counts.values().any(|&count| count == 5); 
}

fn has_straight(hand: &Vec<&Card>) -> bool {
    // Step 1: Collect the numeric values from the hand
    let mut values: Vec<u8> = hand.iter().map(|card| card.value_numeric).collect();

    // Step 2: Sort the values and remove duplicates
    values.sort_unstable();
    values.dedup();

    // Step 3: Handle the Ace as both high (14) and low (1)
    if values.contains(&14) {
        // Check for Ace-low straight (Ace counted as 1)
        let mut values_with_ace_low = values.clone();
        for val in values_with_ace_low.iter_mut() {
            if *val == 14 {
                *val = 1;
            }
        }
        values_with_ace_low.sort_unstable();
        values_with_ace_low.dedup();
        if has_consecutive_sequence(&values_with_ace_low) {
            return true;
        }
    }

    // Step 4: Check for regular straight (Ace counted as high)
    has_consecutive_sequence(&values)
}

fn has_consecutive_sequence(values: &Vec<u8>) -> bool {
    // For a straight, we need at least 5 unique consecutive values
    if values.len() < 5 {
        return false;
    }

    // Check if any sequence of 5 consecutive numbers exists
    for i in 0..=(values.len() - 5) {
        let slice = &values[i..(i + 5)];
        if is_consecutive(slice) {
            return true;
        }
    }
    false
}

fn is_consecutive(slice: &[u8]) -> bool {
    for i in 0..(slice.len() - 1) {
        if slice[i + 1] != slice[i] + 1 {
            return false;
        }
    }
    true
}

fn has_trips(hand: &Vec<&Card>) -> bool {
    let mut value_counts = HashMap::new();

    for card in hand {
        *value_counts.entry(&card.value).or_insert(0) += 1;
    }

    return value_counts.values().any(|&count| count == 3);
}

fn has_two_pair(hand: &Vec<&Card>) -> bool {
    let mut value_counts = HashMap::new();

    for card in hand {
        *value_counts.entry(&card.value).or_insert(0) += 1;
    }

    let pair_count = value_counts.values().filter(|&&count| count == 2).count();

    return pair_count == 2;
}

fn has_pair(hand: &Vec<&Card>) -> bool {
    let mut value_counts = HashMap::new();

    for card in hand {
        *value_counts.entry(&card.value).or_insert(0) += 1;
    };

    return value_counts.values().any(|&count| count == 2);
}

pub fn score_hand(hand: Vec<Card>, table: Vec<Card>) -> f64 {
    let mut scores: Vec<f64> = vec![];
    let all_cards: Vec<Card> = hand.into_iter().chain(table.into_iter()).collect();

    for option in all_cards.iter().combinations(5) {
        let mut score: f64 = 0.0;
        let mut option_new = option.clone();
        sort_cards_by_value_desc(&mut option_new);

        // part 1: calculate first part of score (before decimal point) by determining hand type
        if has_straight_flush(&option_new) {
            score += 8.0;
        }
        else if has_quads(&option_new) {
            score += 7.0;
        }
        else if has_full_house(&option_new) {
            score += 6.0;
        }
        else if has_flush(&option_new) {
            score += 5.0;
        }
        else if has_straight(&option_new) {
            score += 4.0;
        }
        else if has_trips(&option_new) {
            score += 3.0;
        }
        else if has_two_pair(&option_new) {
            score += 2.0;
        }
        else if has_pair(&option_new) {
            score += 1.0;
        }
        // high card
        else {
            score += 0.0;
        }

        // part 2: calculate the second part of the score based on cards in descending order
        score += ((option_new[0].value_numeric as f64 / 100_f64) + (option_new[1].value_numeric as f64 / 100_f64.powf(2.0)) + (option_new[2].value_numeric as f64  / 100_f64.powf(3.0)) + (option_new[3].value_numeric as f64 / 100_f64.powf(4.0)) + (option_new[4].value_numeric as f64 / 100_f64.powf(5.0))) as f64;

        scores.push(score);
    }

    return scores.iter().cloned().reduce(f64::max).unwrap();
}