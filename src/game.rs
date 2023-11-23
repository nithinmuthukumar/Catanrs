use rand::Rng;

use crate::{board::Board, player::Player, resource::ResourceGroup};

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    board: Board,
    bank: ResourceGroup,
    player_with_road: Option<u32>,
    player_with_army: Option<u32>,
}
impl Game {
    pub fn new(players: Vec<Player>, board: Board) -> Self {
        Self {
            players,
            board,
            bank: ResourceGroup::new(),
            player_with_road: None,
            player_with_army: None,
        }
    }
    pub fn give_resources_for_roll(&mut self, roll: i32) {
        if roll == 7 {
            dbg!("7!");
            return;
        }
        let player_yields = self.board.yield_for_roll(roll);
        for (index, res) in player_yields {
            self.players[index].resources += res;
        }
    }
    pub fn roll() -> i32 {
        let mut rng = rand::thread_rng();

        rng.gen_range(1..=6) + rng.gen_range(1..=6)
    }
}
#[cfg(test)]
mod test {
    use crate::axial::Axial;

    use super::*;
    #[test]
    fn test_roll() {
        // Arrange
        let mut game = Game::new(Player::init_players(4), Board::new()); // You should implement a new method for Game struct
        game.board.place_building(
            &game.players[0],
            &Axial::new(0, 1),
            crate::hex::BuildType::Settlement,
            false,
        );
        dbg!(&game.board.vertices);

        // Act
        game.give_resources_for_roll(3);

        // Assert or perform checks based on the expected behavior
        // In this example, you might check that the player's resources were updated correctly after a roll.
        for player in &game.players {
            dbg!(player);
        }
        assert!(false)
    }
}
