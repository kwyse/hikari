use component::Component;
use storage::sequence::SequenceStorage;

pub struct World {
    positions: SequenceStorage,
    velocities: SequenceStorage,
}

impl World {
    pub fn new() -> Self {
        Self {
            positions: SequenceStorage::new(),
            velocities: SequenceStorage::new(),
        }
    }

    pub fn create_entity(&mut self) -> EntityBuilder {
        EntityBuilder::new(self)
    }
}

pub struct EntityBuilder<'a> {
    world: &'a mut World,
    components: Vec<Component>,
    index: usize,
}

impl<'a> EntityBuilder<'a> {
    fn new(world: &'a mut World) -> Self {
        Self {
            world,
            components: Vec::new(),
            index: 0,
        }
    }

    pub fn with_component(mut self, component: Component) -> Self {
        match component {
            Component::Position(_, _) => self.index = self.index.max(self.world.positions.size()),
            Component::Velocity(_, _) => self.index = self.index.max(self.world.velocities.size()),
            _ => { },
        }

        self.components.push(component);
        self
    }

    pub fn build(self) {
        for component in self.components.into_iter() {
            match component {
                Component::Position(_, _) => self.world.positions.add(self.index, component),
                Component::Velocity(_, _) => self.world.velocities.add(self.index, component),
                _ => { },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_entity_with_single_component() {
        let mut world = World::new();
        world.create_entity().with_component(Component::Position(1.0, 0.0)).build();
        assert_eq!(world.positions.as_slice(), [Component::Position(1.0, 0.0)]);
    }

    #[test]
    fn create_entity_with_multiple_components_with_aligned_storage() {
        let mut world = World::new();
        world.create_entity()
            .with_component(Component::Velocity(2.0, 0.0))
            .with_component(Component::Position(5.0, 0.0))
            .build();

        assert_eq!(world.velocities.as_slice(), [Component::Velocity(2.0, 0.0)]);
        assert_eq!(world.positions.as_slice(), [Component::Position(5.0, 0.0)]);
    }

    #[test]
    fn create_entity_with_multiple_components_with_unaligned_storage() {
        let mut world = World::new();
        world.positions.add(0, Component::Position(10.0, 10.0));
        world.create_entity()
            .with_component(Component::Velocity(2.0, 0.0))
            .with_component(Component::Position(5.0, 0.0))
            .build();

        assert_eq!(world.velocities.as_slice(), [Component::Empty, Component::Velocity(2.0, 0.0)]);
        assert_eq!(world.positions.as_slice(), [
            Component::Position(10.0, 10.0), Component::Position(5.0, 0.0)
        ]);
    }
}