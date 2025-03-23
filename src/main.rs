mod card;
mod ranker;
mod simulate;
mod argparser;

// external imports
use clap::Parser;
// internal imports
use crate::card::Card;
use crate::simulate::simulate_both;
use crate::simulate::simulate_no_p2;
use crate::simulate::simulate_no_board;
use crate::simulate::simulate_neither;
use crate::argparser::ArgType;
use crate::argparser::Args;
use crate::argparser::convert_args_to_tuple;

fn main() {
    // get user input from CLI
    let args = Args::parse();
    // get proper input values from args
    let (arg, h1, h2, table, n) = convert_args_to_tuple(&args);

    // simulation time
    println!("[Ag] running {} simulations...", n);
    let w1: u32;
    let w2: u32;
    match arg {
        ArgType::SimulateBoth => { // if both p2 and table are known, no simulations are necessary
            (w1, w2) = simulate_both(h1.clone(), h2.clone(), table.clone(), n);
        },
        ArgType::SimulateNoBoard => {
            (w1, w2) = simulate_no_board(h1.clone(), h2.clone(), n);
        },
        ArgType::SimulateNoP2 => {
            (w1, w2) = simulate_no_p2(h1.clone(), table.clone(), n);
        },
        ArgType::SimulateNeither => {
            (w1, w2) = simulate_neither(h1.clone(), n);
        }
    }
    println!("[Ag] Player 1 wins {} times out of {}", w1, n);
    println!("[Ag] Player 2 wins {} times out of {}", w2, n);
    println!("[Ag] There was a tie {} times out of {}", n - w1 - w2, n);
}
