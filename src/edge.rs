use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use super::axial::Axial;

#[derive(Debug)]
pub struct Edge {
    pub path_coords: PathCoords,
    pub path_type: PathType,
    pub owner: Option<usize>,
}

#[derive(Debug)]
pub enum PathType {
    Road,
    None,
}

impl Edge {
    pub fn new(path_coords: PathCoords, path_type: PathType) -> Self {
        Edge {
            path_coords,
            path_type,
            owner: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PathCoords {
    a: Axial,
    b: Axial,
}
impl PathCoords {
    pub fn new(a: Axial, b: Axial) -> PathCoords {
        if a < b {
            PathCoords { a, b }
        } else {
            PathCoords { a: b, b: a }
        }
    }
    pub fn contains(&self, c: Axial) -> bool {
        self.a == c || self.b == c
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_coords_equality() {
        let coords1 = PathCoords::new(Axial::new(1, 2), Axial::new(3, 4));
        let coords2 = PathCoords::new(Axial::new(3, 4), Axial::new(1, 2));
        dbg!(coords1.clone());
        dbg!(coords2.clone());

        assert_eq!(coords1, coords2);
    }

    #[test]
    fn test_path_coords_hash_equality() {
        let coords1 = PathCoords::new(Axial::new(1, 2), Axial::new(3, 4));
        let coords2 = PathCoords::new(Axial::new(3, 4), Axial::new(1, 2));

        let mut map = std::collections::HashMap::new();
        map.insert(coords1.clone(), "Hello");

        assert_eq!(map.get(&coords2), Some(&"Hello"));
    }
}
