use anyhow::Result;
use rand::Rng;

use crate::{board::Board, phase::Phase, player::Player, resource::ResourceGroup};

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
            bank: ResourceGroup::new(20, 20, 20, 20, 20),
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
    pub fn play() -> Result<()> {
        let mut phase = Phase::START_GAME;

        Ok(())
    }
}
#[cfg(test)]
mod test {
    use crate::{
        axial::Axial,
        edge::{PathCoords, PathType},
        vertex::BuildType,
    };

    use super::*;
    #[test]
    fn test_roll() {
        // Arrange
        let mut game = Game::new(Player::init_players(4), Board::new()); // You should implement a new method for Game struct
        game.board
            .place_building(0, Axial::new(0, 1), BuildType::Settlement, false);
        dbg!(&game.board.vertices);

        // Act
        game.give_resources_for_roll(3);

        // Assert or perform checks based on the expected behavior
        // In this example, you might check that the player's resources were updated correctly after a roll.
        for player in &game.players {
            dbg!(player);
        }
        let mut res = ResourceGroup::empty();
        res.add_resource(crate::resource::Resource::Wood, 1);
        assert_eq!(game.players[0].resources, res)
    }
    #[test]
    fn test_place_building() {
        let mut game = Game::new(Player::init_players(4), Board::new()); // You should implement a new method for Game struct

        let b = game
            .board
            .place_building(0, Axial::new(0, 2), BuildType::Settlement, false);
        match b {
            Ok(_) => assert!(game.board.vertices[&Axial::new(0, 2)]
                .owner
                .is_some_and(|x| x == 0)),
            Err(e) => {
                eprintln!("Error {}", e);
                assert!(false)
            }
        }
        let c = game
            .board
            .place_building(0, Axial::new(0, 2), BuildType::City, false);
        match c {
            Ok(_) => {
                assert!(game.board.vertices[&Axial::new(0, 2)]
                    .owner
                    .is_some_and(|x| x == 0));
                assert_eq!(
                    game.board.vertices[&Axial::new(0, 2)].build_type,
                    BuildType::City
                );
            }
            Err(e) => {
                eprintln!("Error {}", e);
                assert!(false)
            }
        }
    }
    #[test]
    fn test_place_path() {
        let mut game = Game::new(Player::init_players(4), Board::new());
        let path = PathCoords::new(Axial::new(1, 0), Axial::new(0, 1));
        let b = game
            .board
            .place_path(&game.players[0], path.clone(), PathType::Road);
        match b {
            Ok(_) => assert!(game.board.edges[&path].owner.is_some_and(|x| x == 0)),
            Err(e) => {
                assert!(false)
            }
        }
    }
}
