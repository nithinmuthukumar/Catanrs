use crate::{board::Board, player::Player, resource::ResourceGroup};

pub struct Game {
    players: Vec<Player>,
    board: Board,
    bank: ResourceGroup,
    player_with_road: Option<u32>,
    player_with_army: Option<u32>,
}
impl Game {
    pub fn roll() {
        todo!()
    }
}
