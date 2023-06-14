use crate::card::Card;
use rand::Rng;


pub struct Deck {
    pub deck: [Card; 52],
    pub pos: u8,
}

impl Deck {
    fn randomize(mut deck: [Card; 52]) -> [Card; 52] {
        let mut rng = rand::thread_rng();
        for i in 0..52 {
            let pos = rng.gen_range(0..52);
            let temp = deck[i];
            deck[i] = deck[pos as usize];
            deck[pos as usize] = temp;
        }
        return deck;
    }
    pub fn new() -> Self {
        let mut deck: [Card; 52] = [Card::default(); 52];
        for i in 0..52 {
            let new_card: Card = Card {
                suit: if i < 13 { 'D' }
                      else if i < 26 { 'H' }
                      else if i < 39 { 'S' }
                      else if i < 52 { 'C' }
                      else { 'X' },
                      rank: i % 13
            };
            deck[i as usize] = new_card;
        }
        deck = Self::randomize(deck);
        let pos: u8 = 0;
        return Self {deck, pos};
    }
    pub fn pop(&mut self) -> Card {
        self.pos += 1;
        return self.deck[(self.pos - 1) as usize];
    }
}

impl Default for Deck {
    fn default() -> Self {
        return Self {deck: [Card::default(); 52], pos: 0}
    }
}
