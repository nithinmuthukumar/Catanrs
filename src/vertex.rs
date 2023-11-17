use super::{axial::Axial, hex::BuildType};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Axial,
    pub build_type: BuildType,
    pub owner: Option<usize>,
}
impl Vertex {
    pub fn new(pos: Axial, build_type: BuildType) -> Self {
        Vertex {
            pos,
            build_type,
            owner: None,
        }
    }
}
