use crate::{axial::Axial, edge::PathCoords, resource::ResourceGroup, vertex::Vertex};
type PlayerId = usize;
#[derive(Debug)]
pub struct Player {
    pub id: PlayerId,
    pub buildings: Vec<Axial>,
    pub paths: Vec<PathCoords>,
    pub resources: ResourceGroup,
}
impl Player {
    pub fn new(id: usize) -> Self {
        Player {
            id,
            buildings: Vec::new(),
            paths: Vec::new(),
            resources: ResourceGroup::empty(),
        }
    }
    pub fn init_players(size: usize) -> Vec<Player> {
        Vec::from_iter((0..size).map(|i| Player::new(i)))
    }
}
