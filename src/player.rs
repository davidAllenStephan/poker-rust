use crate::card::Card;

#[derive(Copy, Clone)]
pub struct Player {
    pub id: u8,
    pub hand: [Card; 2]
}

impl Player {
    pub fn new(id: u8, hand: [Card; 2]) -> Self {
        Self {id, hand}
    }
}

impl Default for Player {
   fn default() -> Self {
       Self {
           id: 0,
           hand: [Card::default(), Card::default()]
       }
   }
}
