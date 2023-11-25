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

#[derive(Debug, PartialEq, Eq)]
pub struct ResourceGroup {
    resources: HashMap<Resource, i32>,
}
impl ResourceGroup {
    pub fn empty() -> Self {
        ResourceGroup::new(0, 0, 0, 0, 0)
    }

    pub(crate) fn add_resource(&mut self, resource_type: Resource, amount: i32) {
        self.resources
            .entry(resource_type)
            .and_modify(|e| *e += amount);
    }

    fn new(ore: i32, wheat: i32, sheep: i32, brick: i32, wood: i32) -> Self {
        let mut resources = HashMap::new();
        resources.insert(Resource::Ore, ore);
        resources.insert(Resource::Wheat, wheat);
        resources.insert(Resource::Sheep, sheep);
        resources.insert(Resource::Brick, brick);
        resources.insert(Resource::Wood, wood);
        Self { resources }
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
        let mut resource_group = ResourceGroup::empty();
        resource_group.add_resource(Resource::Wood, 3);

        assert_eq!(resource_group.resources[&Resource::Wood], 3);
    }

    #[test]
    fn test_add_assign() {
        let mut resource_group1 = ResourceGroup::empty();
        let mut resource_group2 = ResourceGroup::empty();

        resource_group1.add_resource(Resource::Wood, 3);
        resource_group1.add_resource(Resource::Ore, 1);

        resource_group2.add_resource(Resource::Wood, 2);
        resource_group2.add_resource(Resource::Ore, 2);

        resource_group1 += resource_group2;

        assert_eq!(resource_group1.resources[&Resource::Wood], 5);
        assert_eq!(resource_group1.resources[&Resource::Ore], 3);
    }
}
