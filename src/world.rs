use component::Component;
use storage::map::MapStorage;
use storage::sequence::SequenceStorage;

pub struct World {
    pub commands: MapStorage,
    pub keys: MapStorage,
    pub positions: SequenceStorage,
    pub velocities: SequenceStorage,

    player_id: Option<usize>,
}

impl World {
    pub fn new() -> Self {
        Self {
            commands: MapStorage::new(),
            keys: MapStorage::new(),
            positions: SequenceStorage::new(),
            velocities: SequenceStorage::new(),

            player_id: None,
        }
    }

    pub fn create_entity(&mut self) -> EntityBuilder {
        EntityBuilder::new(self)
    }

    pub fn player_id(&self) -> Option<usize> {
        self.player_id
    }
}

pub struct EntityBuilder<'a> {
    world: &'a mut World,
    components: Vec<Component>,
    index: usize,
    is_player: bool,
}

impl<'a> EntityBuilder<'a> {
    fn new(world: &'a mut World) -> Self {
        Self {
            world,
            components: Vec::new(),
            index: 0,
            is_player: false,
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

    pub fn make_player(mut self) -> Self {
        self.is_player = true;
        self
    }

    pub fn build(self) {
        for component in self.components.into_iter() {
            match component {
                Component::Position(_, _) => self.world.positions.add(self.index, component),
                Component::Velocity(_, _) => self.world.velocities.add(self.index, component),
                Component::KeysPressed(_) => self.world.keys.add(self.index, component),
                Component::Commands(_) => self.world.commands.add(self.index, component),
                _ => { },
            }
        }

        if self.is_player {
            self.world.player_id = Some(self.index);
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
        assert_eq!(world.player_id, None);
        assert_eq!(world.positions.as_slice(), [Component::Position(1.0, 0.0)]);
    }

    #[test]
    fn create_entity_with_multiple_components_with_aligned_storage() {
        let mut world = World::new();
        world.create_entity()
            .with_component(Component::Velocity(2.0, 0.0))
            .with_component(Component::Position(5.0, 0.0))
            .build();

        assert_eq!(world.player_id, None);
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

        assert_eq!(world.player_id, None);
        assert_eq!(world.velocities.as_slice(), [Component::Empty, Component::Velocity(2.0, 0.0)]);
        assert_eq!(world.positions.as_slice(), [
            Component::Position(10.0, 10.0), Component::Position(5.0, 0.0)
        ]);
    }

    #[test]
    fn create_player_entity() {
        let mut world = World::new();
        world.positions.add(0, Component::Position(10.0, 10.0));
        world.create_entity()
            .with_component(Component::Velocity(2.0, 0.0))
            .with_component(Component::Position(5.0, 0.0))
            .make_player()
            .build();

        assert_eq!(world.player_id, Some(1));
        assert_eq!(world.velocities.as_slice(), [Component::Empty, Component::Velocity(2.0, 0.0)]);
        assert_eq!(world.positions.as_slice(), [
            Component::Position(10.0, 10.0), Component::Position(5.0, 0.0)
        ]);
    }
}
