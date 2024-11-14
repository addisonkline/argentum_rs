mod card;
mod ranker;

use card::Card;
use ranker::score_hand;

fn determine_winner(p1: Vec<Card>, p2: Vec<Card>, table: Vec<Card>) -> (u8, u8) {
    let score_1 = score_hand(p1, table);
    let score_2 = score_hand(p2, table);

    let mut win_1: u8 = 0;
    let mut win_2: u8 = 0;

    if score_1 >= score_2 {
        win_1 += 1;
    }
    if score_2 >= score_1 {
        win_2 += 1;
    }

    return (win_1, win_2)
}

fn simulate(p1: Vec<Card>, table: Vec<Card>, n: u32) -> (u32, u32) {
    let mut wins_1: u32 = 0;
    let mut wins_2: u32 = 0;
    
    for iteration in 0..n {
        // TODO

        // determine winner for this iterations
        let win_1, win_2 = determine_winner(p1, p2, table);
        wins_1 += win_1;
        wins_2 += win_2;
    }

    return (wins_1, wins_2)
}