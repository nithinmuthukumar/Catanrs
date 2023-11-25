use crate::resource::ResourceGroup;

use super::axial::Axial;

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
#[derive(Debug, Clone, Copy)]
pub enum BuildType {
    City,
    Settlement,
    None,
}
impl BuildType {
    pub fn cost(&self) -> ResourceGroup {
        match self {
            BuildType::City => ResourceGroup::empty(),
            BuildType::Settlement => ResourceGroup::empty(),
            BuildType::None => ResourceGroup::empty(),
        }
    }
}
