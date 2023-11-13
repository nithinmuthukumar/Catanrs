use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{
    harbor::Harbor,
    hex::Hex,
    hex::{BuildType, Resource},
};

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
                vertices.insert(a, Vertex::new(a, BuildType::None, -1));
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
        let mut valid = Vec::new();
        for v in self.vertices.values() {
            if self.is_valid_settlement_coords(v) {
                valid.push(v);
            }
        }
        valid
    }
    pub fn get_adjacent_vertices(&self, v: &Vertex) -> Vec<&Vertex> {
        let mut valid = Vec::new();
        for &offset in OFFSETS.iter() {
            let adjacent = &(v.pos + offset);
            if self.vertices.contains_key(adjacent) {
                valid.push(&self.vertices[adjacent]);
            }
        }
        valid
    }

    /// Checks if a vertex is available for settlement
    pub fn is_valid_settlement_coords(&self, v: &Vertex) -> bool {
        if v.owner != -1 {
            return false;
        }
        for a in self.get_adjacent_vertices(v) {
            if a.owner != -1 {
                return false;
            }
        }
        true
    }
}
