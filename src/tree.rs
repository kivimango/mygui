use std::{collections::HashMap, ops::Index};
use specs::Entity;
use indextree::{Arena, Children, Node, NodeError, NodeId};

/// The index of the entity in the tree's arena.
/// Wrapper over indextree::NodeId
pub struct TreeIdx(pub NodeId);

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
    pub fn add_node(&mut self, node: Entity) -> TreeIdx {
        let idx = self.arena.new_node(node);
        self.map.insert(node, idx);
        TreeIdx {0: idx}
    }

    /// Appends `child` to `parent` in the tree.
    ///
    /// # Panics
    /// This method is panics if the two node are the same, 
    /// or any of the nodes are removed.
    /// 
    /// For the non-panicking version, use try_append_child().
    pub fn append_child(&mut self, parent: TreeIdx, child: TreeIdx) {
        parent.0.append(child.0, &mut self.arena);
    }

    pub fn count(&self) -> usize {
        self.arena.count()
    }

    pub fn child(&self, parent: TreeIdx) -> Children<Entity> {
        parent.0.children(&self.arena)
    }

    pub fn children(&self) -> impl Iterator<Item = &Node<Entity>> {
        self.arena.iter()
    }

    /* pub fn id(&self, node: Entity) -> NodeId {
       
    } */

    pub fn is_empty(&self) -> bool {
        self.arena.is_empty()
    }

    pub fn root(&self) -> Option<Entity> {
        self.root
    }

    pub fn root_idx(&self) -> Option<TreeIdx> {
        if let Some(root) = self.root() {
            if let Some(id) = self.map.get(&root) {
                return Some(TreeIdx {0: *id})
            }
            return None
        }
        None
    }

    pub fn set_root(&mut self, root: Entity) -> TreeIdx {
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
}

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
