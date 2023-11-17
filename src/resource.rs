use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub enum Resource {
    None,
    Ore,
    Wheat,
    Sheep,
    Brick,
    Wood,
}

pub struct ResourceGroup {
    resources: HashMap<Resource, u32>,
}
impl ResourceGroup {
    pub fn new() -> ResourceGroup {
        ResourceGroup {
            resources: HashMap::new(),
        }
    }
}
