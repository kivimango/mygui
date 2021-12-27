use std::{collections::HashMap};
use specs::Entity;
use indextree::{Arena, Children, Descendants, NodeError, NodeId};

pub(crate) struct EntityTree {
    arena: Arena<Entity>,
    root: Option<Entity>,
    map: HashMap<Entity, NodeId>
}

impl EntityTree {
    pub(crate) fn new() -> EntityTree {
        EntityTree {
            arena: Arena::new(),
            root: None,
            map: HashMap::new()
        }
    }

    /// Registers the node's entity in the tree and returns its id.
    /// The node does not have relationship with other nodes unless you call append_child().
    /// If the node's entity is already added to the tree, nothing happens.
    /// For the alternate version of the method, use try_add_node().
    pub fn add_node(&mut self, node: Entity) {
        if !self.map.contains_key(&node) {
            let idx = self.arena.new_node(node);
            self.map.insert(node, idx);
        }
    }

    /// Makes a parent-child relationship between two nodes in the tree.
    /// Both nodes must be added to the tree before calling this method!
    ///
    /// # Panics
    /// This method is panics if the two node are the same, 
    /// or any of the nodes are removed.
    /// 
    /// For the non-panicking version, use try_append_child().
    pub fn append_child(&mut self, parent: Entity, child: Entity) {
        let parent_node = self.map.get(&parent)
            .expect("Cannot append: The parent node not found! Add the node to the tree with add_node()!");
        let child_node = self.map.get(&child)
            .expect("Cannot append: The child node not found! Add the node to the tree with add_node()!");
        parent_node.append(*child_node, &mut self.arena);
    }

    pub fn count(&self) -> usize {
        self.arena.count()
    }

    pub fn child_of(&self, parent: Entity) -> Option<Entity> {
        if let Some(node_id) = self.map.get(&parent) {
            if let Some(child) = node_id.children(&self.arena).nth(0) {
                if let Some(node) = self.arena.get(child) {
                    return Some(*node.get())
                } else {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    pub fn children_of(&self, parent: Entity) -> Children<Entity> {
        let node = self.map.get(&parent);
        node.unwrap().children(&self.arena)
    }

    pub fn children(&self) -> Descendants<Entity> {
        let root = self.map.get(&self.root.unwrap()).unwrap();
        root.descendants(&self.arena)
    }

    pub fn entitiy_of(&self, node: NodeId) -> Option<Entity> {
        if let Some(node) = self.arena.get(node) {
            return Some(*node.get());
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.arena.is_empty()
    }

    pub fn parent(&self, widget: Entity) -> Option<Entity> {
        if let Some(node_idx) = self.map.get(&widget) {
            if let Some(_i) = self.arena.get(*node_idx) {
                if let Some(parent_idx) = node_idx.ancestors(&self.arena).skip(1).next() {
                    if let Some(parent_entity) = self.arena.get(parent_idx) {
                        return Some(*parent_entity.get())
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            return  None;
        }
    }

    pub fn root(&self) -> Option<Entity> {
        self.root
    }

    pub fn set_root(&mut self, root: Entity) {
        /* 
        if let Some(idx) = self.map.get(&root) {
            if let Some(node) = self.arena.get(*idx) {
                self.root = Some(root);
            } else {
                self.root = Some(root);
                self.add_node(root);
            }
        }*/
        self.root = Some(root);
        self.add_node(root)
    }

    pub fn try_append_child(&mut self, parent: NodeId, child: NodeId) -> Result<(), NodeError> {
        parent.checked_append(child, &mut self.arena)
    }

    pub fn try_children_of(&self, parent: Entity) -> Option<Children<Entity>> {
        if let Some(node) = self.map.get(&parent) {
            return Some(node.children(&self.arena))
        }
        None
    }

    /* fn valid(&self, parent: &Entity, child: &Entity) -> bool {
        self.map.contains_key(parent) && self.map.contains_key(child)
    } */
}

/*pub struct TreeIter {
    stack: Vec<Entity>
}

impl TreeIter {
    fn new() -> TreeIter {

    }
}

impl Iterator for TreeIter {
    fn 
}*/

mod test {
    use crate::{EntityTree};
    use specs::{Builder, Entity, World, WorldExt};
    use std::collections::HashMap;

    /*#[test]
    fn test_new() {
        let mut world = World::new();
        let entity = world.create_entity().build();
        world.maintain();
        let tree = EntityTree::new();

        assert_eq!(entity, tree.root.entity);
        assert_eq!(1, tree.len());
    }*/
}
