use itertools::Itertools;
use crate::card::{
    Card,
    generate_deck,
    shuffle_deck
};
use crate::ranker::score_hand;

fn determine_winner(p1: Vec<Card>, p2: Vec<Card>, table: Vec<Card>) -> (u8, u8) {
    let score_1 = score_hand(p1.clone(), table.clone());
    let score_2 = score_hand(p2.clone(), table.clone());

    if score_1 > score_2 {
        return (1, 0);
    }
    if score_2 > score_1 {
        return (0, 1);
    }

    // tie
    return (0, 0);
}

pub fn simulate_no_p2(p1: Vec<Card>, table: Vec<Card>, n: u32) -> (u32, u32) {
    let mut wins_1: u32 = 0;
    let mut wins_2: u32 = 0;
    
    for iteration in 0..n {
        // generate initial deck
        let mut deck = generate_deck();
        shuffle_deck(&mut deck);

        // determine what cards are remaining
        // 1. filter out p1 hand
        let mut deck_remaining: Vec<Card> = deck.clone()
            .into_iter()
            .filter(|item| !p1.contains(item))
            .collect();
        // 2. filter out table cards
        deck_remaining = deck.clone()
            .into_iter()
            .filter(|item| !table.contains(item))
            .collect();
        
        let p2: Vec<Card> = vec![
            Card::new(deck_remaining[0].value.clone(), deck_remaining[0].suit.clone()),
            Card::new(deck_remaining[1].value.clone(), deck_remaining[1].suit.clone())
            ];

        // determine winner for this iterations
        let (win_1, win_2) = determine_winner(p1.clone(), p2.clone(), table.clone());
        wins_1 += win_1 as u32;
        wins_2 += win_2 as u32;
    }

    return (wins_1, wins_2)
}