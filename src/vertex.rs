use std::any;

use anyhow::{anyhow, Result};

use crate::{board::Board, resource::ResourceGroup};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildType {
    City,
    Settlement,
    None,
}
impl BuildType {
    pub fn cost(&self) -> ResourceGroup {
        match self {
            BuildType::City => ResourceGroup::new(0, 1, 1, 1, 1),
            BuildType::Settlement => ResourceGroup::new(3, 2, 0, 0, 0),
            BuildType::None => ResourceGroup::empty(),
        }
    }
}
