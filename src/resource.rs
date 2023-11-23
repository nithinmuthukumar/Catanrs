use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Resource {
    None,
    Ore,
    Wheat,
    Sheep,
    Brick,
    Wood,
}

#[derive(Debug)]
pub struct ResourceGroup {
    resources: HashMap<Resource, i32>,
}
impl ResourceGroup {
    pub fn new() -> ResourceGroup {
        ResourceGroup {
            resources: HashMap::new(),
        }
    }

    pub(crate) fn add_resource(&mut self, resource_type: Resource, amount: i32) {
        *self.resources.entry(resource_type).or_insert(0) += amount;
    }
}
impl std::ops::AddAssign<ResourceGroup> for ResourceGroup {
    fn add_assign(&mut self, rhs: ResourceGroup) {
        for (resource_type, amount) in rhs.resources {
            self.add_resource(resource_type, amount);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_resource_single() {
        let mut resource_group = ResourceGroup::new();
        resource_group.add_resource(Resource::Wood, 3);

        assert_eq!(resource_group.resources[&Resource::Wood], 3);
    }

    #[test]
    fn test_add_assign() {
        let mut resource_group1 = ResourceGroup::new();
        let mut resource_group2 = ResourceGroup::new();

        resource_group1.add_resource(Resource::Wood, 3);
        resource_group1.add_resource(Resource::Ore, 1);

        resource_group2.add_resource(Resource::Wood, 2);
        resource_group2.add_resource(Resource::Ore, 2);

        resource_group1 += resource_group2;

        assert_eq!(resource_group1.resources[&Resource::Wood], 5);
        assert_eq!(resource_group1.resources[&Resource::Ore], 3);
    }
}
