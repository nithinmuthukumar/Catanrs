use std::collections::HashMap;

use anyhow::{anyhow, Error, Result};
use lazy_static::lazy_static;

use crate::{harbor::Harbor, hex::BuildType, hex::Hex, player::Player, resource::Resource};

use super::{
    axial::Axial,
    edge::{Edge, EdgeCoords},
    vertex::Vertex,
};

lazy_static! {
    static ref OFFSETS: [Axial; 6] = [
        Axial::new(1, 0),
        Axial::new(0, 1),
        Axial::new(-1, 1),
        Axial::new(-1, 0),
        Axial::new(0, -1),
        Axial::new(1, -1),
    ];
}
pub struct Board {
    pub hexes: HashMap<Axial, Hex>,
    pub edges: HashMap<EdgeCoords, Edge>,
    pub vertices: HashMap<Axial, Vertex>,
    pub harbors: HashMap<EdgeCoords, Harbor>,
    pub robber: Axial,
}
impl Board {
    pub fn new() -> Self {
        let mut hex_data: HashMap<Axial, (Resource, i32)> = HashMap::new();
        hex_data.insert(Axial::new(4, -2), (Resource::Ore, 10));
        hex_data.insert(Axial::new(3, 0), (Resource::Sheep, 2));
        hex_data.insert(Axial::new(2, 2), (Resource::Wood, 9));

        hex_data.insert(Axial::new(3, -3), (Resource::Wheat, 12));
        hex_data.insert(Axial::new(2, -1), (Resource::Brick, 6));
        hex_data.insert(Axial::new(1, 1), (Resource::Sheep, 4));
        hex_data.insert(Axial::new(0, 3), (Resource::Brick, 10));

        hex_data.insert(Axial::new(2, -4), (Resource::Wheat, 9));
        hex_data.insert(Axial::new(1, -2), (Resource::Wood, 11));
        hex_data.insert(Axial::new(0, 0), (Resource::None, 0));
        hex_data.insert(Axial::new(-1, 2), (Resource::Wood, 3));
        hex_data.insert(Axial::new(-2, 4), (Resource::Ore, 8));

        hex_data.insert(Axial::new(0, -3), (Resource::Wood, 8));
        hex_data.insert(Axial::new(-1, -1), (Resource::Ore, 3));
        hex_data.insert(Axial::new(-2, 1), (Resource::Wheat, 4));
        hex_data.insert(Axial::new(-3, 3), (Resource::Sheep, 5));

        hex_data.insert(Axial::new(-2, -2), (Resource::Brick, 5));
        hex_data.insert(Axial::new(-3, 0), (Resource::Wheat, 6));
        hex_data.insert(Axial::new(-4, 2), (Resource::Sheep, 11));

        let mut hexes: HashMap<Axial, Hex> = HashMap::new();
        for (pos, ru) in hex_data.into_iter() {
            hexes.insert(
                pos,
                Hex {
                    pos,
                    number: ru.1,
                    resource_type: ru.0,
                },
            );
        }
        let mut vertices: HashMap<Axial, Vertex> = HashMap::new();
        let mut robber = Axial::new(0, 0);
        for hex in hexes.values() {
            if hex.number == 0 {
                robber = hex.pos;
            }
            for &offset in OFFSETS.iter() {
                let a: Axial = hex.pos + offset;
                vertices.insert(a, Vertex::new(a, BuildType::None));
            }
        }

        let mut edges: HashMap<EdgeCoords, Edge> = HashMap::new();
        for &v in vertices.keys() {
            for &offset in OFFSETS.iter() {
                let adjacent = v + offset;
                if vertices.contains_key(&adjacent) {
                    let coords = EdgeCoords::new(v, adjacent);
                    let p = Edge::new(coords.clone(), super::edge::EdgeType::None);
                    edges.insert(coords, p);
                }
            }
        }

        let board = Board {
            hexes,
            edges,
            vertices,
            harbors: HashMap::new(),
            robber,
        };
        board
    }
    pub fn get_valid_settlement_coords(&self) -> Vec<&Vertex> {
        self.vertices
            .values()
            .filter(|&v| self.is_valid_settlement_coords(&v.pos))
            .collect()
    }
    pub fn get_adjacent_vertices(&self, v: &Vertex) -> Vec<&Vertex> {
        OFFSETS
            .iter()
            .map(|&offset| v.pos + offset)
            .filter_map(|adjacent| self.vertices.get(&adjacent))
            .collect()
    }

    /// Checks if a vertex is available for settlement
    pub fn is_valid_settlement_coords(&self, pos: &Axial) -> bool {
        if let Some(v) = self.vertices.get(pos) {
            if v.owner.is_some() {
                return false;
            }
            for neighbour in self.get_adjacent_vertices(v) {
                if neighbour.owner.is_some() {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
    pub fn place_building(
        &mut self,
        player: &Player,
        pos: &Axial,
        build_type: BuildType,
        //makes sure there is a road connecting
        ensure_connected: bool,
    ) -> Result<()> {
        self.validate_build(&player, pos, ensure_connected)?;
        if let Some(v) = self.vertices.get_mut(pos) {
            v.build_type = build_type;
            v.owner = Some(player.id);
            return Ok(());
        }
        return Err(anyhow!("Invalid Build"));
    }

    fn validate_build(&self, player: &Player, pos: &Axial, ensure_connected: bool) -> Result<()> {
        if !self.is_valid_settlement_coords(pos) {
            return Err(anyhow!("Settlement spot is not valid"));
        }
        //TODO ensure_connected
        Ok(())
    }
    pub fn yield_for_roll(roll: i32) {
        todo!()
    }
}
#[test]
fn test_place_building() {
    let mut board = Board::new();
    let player = Player::new(1);
    let b = board.place_building(&player, &Axial::new(0, 2), BuildType::Settlement, false);
    match b {
        Ok(_) => assert!(board.vertices[&Axial::new(0, 2)]
            .owner
            .is_some_and(|x| x == 1)),
        Err(e) => {
            eprintln!("Error {}", e);
            assert!(false)
        }
    }
}
