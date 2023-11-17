use crate::resource::Resource;

use super::axial::Axial;

#[derive(Clone, Copy, Debug)]
pub struct Hex {
    pub resource_type: Resource,
    pub number: i32,
    pub pos: Axial,
}

#[derive(Debug, Clone, Copy)]
pub enum BuildType {
    City,
    Settlement,
    None,
}
