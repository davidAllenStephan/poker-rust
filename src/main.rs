mod deck;
mod card;
mod player;

fn main() {
    let mut deck = deck::Deck::new();
    for i in 0..52 {
        let suit = deck.deck[i].suit;
        let rank = deck.deck[i].rank;

        print!("{}", format!("{}{} ", suit, rank));
    }
    println!("");

    deck.pop();
    let c = deck.pop();
    print!("{}", format!("{}{} ", c.suit, c.rank));

}
