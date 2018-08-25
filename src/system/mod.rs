/// Pieces of data that compose an entity
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Component {
    /// The absence of a component
    Empty,
}

/// Enables different collections of components to be iterated over on the same entities
pub trait Join {
    fn join(&mut self) -> Vec<(&mut Component, &Component)>;
}

impl<'a> Join for (&'a mut Vec<Component>, &'a Vec<Component>) {
    fn join(&mut self) -> Vec<(&mut Component, &Component)> {
        self.0.iter_mut().zip(self.1.iter())
            .filter(|(&mut a, &b)| a != Component::Empty && b != Component::Empty)
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_empty_collections() {
        assert_eq!((&mut vec![], &vec![]).join(), vec![]);
    }
}
