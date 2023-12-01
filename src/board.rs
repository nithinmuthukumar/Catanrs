use std::collections::HashMap;

use anyhow::{anyhow, Error, Result};
use lazy_static::lazy_static;

use crate::{
    edge::PathType,
    harbor::Harbor,
    hex::Hex,
    player::Player,
    resource::{Resource, ResourceGroup},
    vertex::BuildType,
};

use super::{
    axial::Axial,
    edge::{Edge, PathCoords},
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
#[derive(Debug)]
pub struct Board {
    pub hexes: HashMap<Axial, Hex>,
    pub edges: HashMap<PathCoords, Edge>,
    pub vertices: HashMap<Axial, Vertex>,
    pub harbors: HashMap<PathCoords, Harbor>,
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

        let mut edges: HashMap<PathCoords, Edge> = HashMap::new();
        for &v in vertices.keys() {
            for &offset in OFFSETS.iter() {
                let adjacent = v + offset;
                if vertices.contains_key(&adjacent) {
                    let coords = PathCoords::new(v, adjacent);
                    let p = Edge::new(coords.clone(), super::edge::PathType::None);
                    edges.insert(coords, p);
                }
            }
            //
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
    pub fn get_adjacent_vertices(&self, a: Axial) -> Vec<&Vertex> {
        OFFSETS
            .iter()
            .map(|&offset| a + offset)
            .filter_map(|adjacent| self.vertices.get(&adjacent))
            .collect()
    }

    pub fn place_building(
        &mut self,
        player: usize,
        pos: Axial,
        build_type: BuildType,
        //makes sure there is a road connecting
        ensure_connected: bool,
    ) -> Result<()> {
        self.validate_build(build_type, player, pos, ensure_connected)?;

        if let Some(v) = self.vertices.get_mut(&pos) {
            v.build_type = build_type;
            v.owner = Some(player);
            return Ok(());
        }
        return Err(anyhow!("Invalid Build"));
    }

    fn validate_build(
        &self,
        build_type: BuildType,
        player: usize,
        pos: Axial,
        ensure_connected: bool,
    ) -> Result<(), Error> {
        match build_type {
            BuildType::City => self.validate_city(player, pos),
            BuildType::Settlement => self.validate_settlement(player, pos, ensure_connected),
            BuildType::None => todo!(),
        }?;
        Ok(())
    }
    pub fn get_valid_build_spots(
        &self,
        build_type: BuildType,
        player: usize,
        ensure_connected: bool,
    ) -> Vec<Axial> {
        self.vertices
            .keys()
            .filter(|&v| {
                self.validate_build(build_type, player, *v, ensure_connected)
                    .is_ok()
            })
            .cloned()
            .collect()
    }

    pub fn yield_for_roll(&self, roll: i32) -> HashMap<usize, ResourceGroup> {
        let mut yields = HashMap::new();
        for hex in self.hexes.values() {
            if hex.number == roll && self.robber != hex.pos {
                for v in self.get_adjacent_vertices(hex.pos) {
                    if let Some(o) = v.owner {
                        let group = yields.entry(o).or_insert(ResourceGroup::empty());
                        group.add_resource(hex.resource_type, 1);
                    }
                }
            }
        }
        yields
    }

    pub fn place_path(
        &mut self,
        player: &Player,
        coords: PathCoords,
        path_type: PathType,
    ) -> Result<()> {
        self.validate_path(player, coords.clone())?;
        if let Some(v) = self.edges.get_mut(&coords) {
            v.path_type = path_type;
            v.owner = Some(player.id);
            return Ok(());
        }
        Err(anyhow!("Invalid Build"))
    }

    fn validate_path(&self, player: &Player, coords: PathCoords) -> Result<()> {
        if !self.is_valid_path_coords(&coords) {
            return Err(anyhow!("Settlement spot is not valid"));
        }
        //TODO ensure connected
        Ok(())
    }

    fn is_valid_path_coords(&self, coords: &PathCoords) -> bool {
        if let Some(v) = self.edges.get(&coords) {
            if v.owner.is_some() {
                return false;
            }
            // for neighbour in self.get_adjacent_vertices(v.) {
            //     if neighbour.owner.is_some() {
            //         return false;
            //     }
            // }
            true
        } else {
            false
        }
    }
    pub fn validate_settlement(
        &self,
        player: usize,
        pos: Axial,
        ensure_connected: bool,
    ) -> Result<()> {
        if let Some(v) = self.vertices.get(&pos) {
            if v.owner.is_some() {
                return Err(anyhow!("Vertex is already owned"));
            }
            for neighbour in self.get_adjacent_vertices(v.pos) {
                if neighbour.owner.is_some() {
                    return Err(anyhow!("Neighbour is occupied {:?}", neighbour));
                }
            }
            Ok(())
        } else {
            return Err(anyhow!("Vertex does not exist"));
        }
    }
    pub fn validate_city(&self, player: usize, pos: Axial) -> Result<()> {
        if let Some(v) = self.vertices.get(&pos) {
            if !v.owner.is_some_and(|owner| owner == player) {
                Err(anyhow!("Vertex is not owned by player"))
            } else if v.build_type != BuildType::Settlement {
                Err(anyhow!("There is no settlement on the vertex"))
            } else {
                Ok(())
            }
        } else {
            Err(anyhow!("Vertex does not exist"))
        }
    }
}
