use crate::utils::print_day_banner;
use crate::utils::read_lines;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Deck = VecDeque<usize>;
type Decks = [Deck; 2];

pub fn day22_01() {
    print_day_banner(22, 1);
    let mut decks = load_decks();
    let winner: usize;

    loop {
        let cards: Vec<usize> = decks
            .iter_mut()
            .map(|deck| deck.pop_front().unwrap())
            .collect();
        if cards[0] > cards[1] {
            decks[0].push_back(cards[0]);
            decks[0].push_back(cards[1]);
        } else {
            decks[1].push_back(cards[1]);
            decks[1].push_back(cards[0]);
        }

        if decks[0].len() == 0 {
            winner = 1;
            break;
        }
        if decks[1].len() == 0 {
            winner = 0;
            break;
        }
    }

    println!("Winner is Player {:?}", winner + 1);

    let score: usize = compute_score(&decks[winner]);
    println!("Winner score: {:?}", score);
}
pub fn day22_02() {
    print_day_banner(22, 2);
    let decks = load_decks();
    let mut memo: HashMap<String, usize> = HashMap::new();
    let winner = play_game(decks, &mut memo);
    // println!("{:?}", winner);

    println!("Winner is Player {:?}", winner.0);
    let score: usize = compute_score(&winner.1);
    println!("Winner score: {:?}", score);
}

fn play_game(decks: Decks, memo: &mut HashMap<String, usize>) -> (usize, Deck) {
    let winner: usize;
    let mut decks = decks;
    let mut previous_decks: HashSet<String> = HashSet::new();
    loop {
        let deck_str = format!("{:?}", decks);
        if !previous_decks.insert(deck_str) {
            winner = 0;
            break;
        }
        let round_winner: usize;
        let cards: Vec<usize> = decks
            .iter_mut()
            .map(|deck| deck.pop_front().unwrap())
            .collect();
        if decks[0].len() >= cards[0] && decks[1].len() >= cards[1] {
            let new_decks: Decks = [
                decks[0].iter().take(cards[0]).cloned().collect(),
                decks[1].iter().take(cards[1]).cloned().collect(),
            ];
            let subresult = play_game(new_decks, memo);
            round_winner = subresult.0;
        } else {
            round_winner = if cards[0] > cards[1] { 0 } else { 1 };
        }
        if round_winner == 0 {
            decks[0].push_back(cards[0]);
            decks[0].push_back(cards[1]);
        } else {
            decks[1].push_back(cards[1]);
            decks[1].push_back(cards[0]);
        }

        if decks[0].len() == 0 {
            winner = 1;
            break;
        }
        if decks[1].len() == 0 {
            winner = 0;
            break;
        }
    }

    return (winner, decks[winner].clone());
}

fn compute_score(deck: &Deck) -> usize {
    return deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, value)| value * (i + 1))
        .sum();
}

fn load_decks() -> Decks {
    let lines: Vec<_> = read_lines("./inputs/22")
        .unwrap()
        .filter_map(Result::ok)
        .collect();
    let mut decks: Decks = [VecDeque::new(), VecDeque::new()];
    let mut deck = 0;
    for line in lines {
        if line.starts_with("Player") {
            continue;
        }
        if line == "" {
            deck += 1;
            continue;
        }
        let value: usize = line.parse().unwrap();
        decks[deck].push_back(value);
    }
    return decks;
}
