use std::collections::HashMap;
use specs::Entity;

pub struct Node {
    pub entity: Entity,
    pub children: HashMap<Entity, Vec<Node>>,
    pub root: Option<Entity>
}

impl Node {
    pub fn new(entity: Entity, root: Option<Entity>) -> Node {
        Node {
            entity,
            children: HashMap::new(),
            root
        }
    }

    pub fn append_child(&mut self, parent: &Entity, node: Node) {
        if self.children.contains_key(&parent) {
            if let Some (children) = self.children.get_mut(&parent) {
                children.push(node);
            }
        }
    }
}

pub struct EntityTree {
    root: Node
}

impl EntityTree {
    pub fn new(root: Node) -> EntityTree {
        assert!(root.root.is_none());

        EntityTree {
            root
        }
    }

    pub fn add_child(&mut self, node: Node) {
        if let Some(children) = self.root.children.get_mut(&self.root.entity) {
            children.push(node);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.root.children.len() + 1 
    }

    pub fn root(&self) -> &Node {
        &self.root
    }

}

mod test {
    use std::collections::HashMap;
    use specs::{Builder, Entity, World, WorldExt};
    use crate::{EntityTree, Node};

    #[test]
    fn test_new() {
        let mut world = World::new();
        let entity = world.create_entity().build();
        world.maintain();
        let root = Node {
            entity: entity,
            children: HashMap::new(),
            root: None
        };
        let tree = EntityTree::new(root);

        assert_eq!(entity, tree.root.entity);
        assert_eq!(1, tree.len());
    }
}