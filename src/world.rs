use component::Component;

pub struct World {
    positions: Vec<Option<Component>>,
    velocities: Vec<Option<Component>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            velocities: Vec::new(),
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
            Component::Position(_, _) => self.index = self.index.max(self.world.positions.len()),
            Component::Velocity(_, _) => self.index = self.index.max(self.world.velocities.len()),
            Component::Empty => { },
        }

        self.components.push(component);
        self
    }

    pub fn build(self) {
        for component in self.components.into_iter() {
            match component {
                Component::Position(_, _) => insert(self.index, &mut self.world.positions, component),
                Component::Velocity(_, _) => insert(self.index, &mut self.world.velocities, component),
                Component::Empty => { },
            }
        }
    }
}

fn insert(index: usize, component_store: &mut Vec<Option<Component>>, component: Component) {
    let allocated = component_store.len();
    if index > allocated {
        component_store.reserve(index - allocated);
    }

    for _ in component_store.len()..index {
        component_store.push(None);
    }

    component_store.insert(index, Some(component));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_entity_with_single_component() {
        let mut world = World::new();
        world.create_entity().with_component(Component::Position(1.0, 0.0)).build();
        assert_eq!(world.positions, vec![Some(Component::Position(1.0, 0.0))]);
    }

    #[test]
    fn create_entity_with_multiple_components_with_aligned_storage() {
        let mut world = World::new();
        world.create_entity()
            .with_component(Component::Velocity(2.0, 0.0))
            .with_component(Component::Position(5.0, 0.0))
            .build();

        assert_eq!(world.velocities, vec![Some(Component::Velocity(2.0, 0.0))]);
        assert_eq!(world.positions, vec![Some(Component::Position(5.0, 0.0))]);
    }

    #[test]
    fn create_entity_with_multiple_components_with_unaligned_storage() {
        let mut world = World::new();
        world.positions.push(Some(Component::Position(10.0, 10.0)));
        world.create_entity()
            .with_component(Component::Velocity(2.0, 0.0))
            .with_component(Component::Position(5.0, 0.0))
            .build();

        assert_eq!(world.velocities, vec![None, Some(Component::Velocity(2.0, 0.0))]);
        assert_eq!(world.positions, vec![
            Some(Component::Position(10.0, 10.0)), Some(Component::Position(5.0, 0.0))
        ]);
    }
}