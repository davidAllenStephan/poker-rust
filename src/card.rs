#[derive(Clone, Copy)]
pub struct Card {
    pub suit: char,
    pub rank: u8,
}

impl Card {
    pub fn new(suit: char, rank: u8) -> Self {
        Self {
            suit,
            rank
        }
    }
}

impl Default for Card {
    fn default() -> Self {
        Self {
            suit: 'X',
            rank: 0,
        }
    }
}
