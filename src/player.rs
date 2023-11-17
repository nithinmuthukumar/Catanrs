use crate::{axial::Axial, edge::EdgeCoords, resource::ResourceGroup, vertex::Vertex};

pub struct Player {
    pub id: usize,
    pub buildings: Vec<Axial>,
    pub paths: Vec<EdgeCoords>,
    pub resources: ResourceGroup,
}
impl Player {
    pub fn new(id: usize) -> Self {
        Player {
            id,
            buildings: Vec::new(),
            paths: Vec::new(),
            resources: ResourceGroup::new(),
        }
    }
}
