use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::path::PathBuf;
use super::file_item::FileItem;

#[derive(Debug)]
pub struct Node {
    pub data: FileItem,
    pub children: Vec<Rc<RefCell<Node>>>,
    pub parent: Option<Weak<RefCell<Node>>>,
}

impl Node {
    pub fn new(data: FileItem) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            data,
            children: Vec::new(),
            parent: None,
        }))
    }

    pub fn add_child(parent: &Rc<RefCell<Self>>, child: Rc<RefCell<Self>>) {
        child.borrow_mut().parent = Some(Rc::downgrade(parent));
        parent.borrow_mut().children.push(child);
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    pub fn get_path_components(&self) -> Vec<String> {
        let mut path = Vec::new();
        path.push(self.data.name().to_string());
        
        let mut current_parent = self.parent.clone();
        while let Some(weak_parent) = current_parent {
            if let Some(parent_rc) = weak_parent.upgrade() {
                let parent = parent_rc.borrow();
                path.push(parent.data.name().to_string());
                current_parent = parent.parent.clone();
            } else {
                break;
            }
        }
        
        path.reverse();
        path
    }

    pub fn get_full_path(&self) -> PathBuf {
        let components = self.get_path_components();
        let mut path = PathBuf::new();
        for component in components {
            path.push(component);
        }
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_structure() {
        let root_item = FileItem::new("root".to_string(), 0, "".to_string(), 1).unwrap();
        let root = Node::new(root_item);

        let child_item = FileItem::new("child".to_string(), 1, "".to_string(), 2).unwrap();
        let child = Node::new(child_item);

        Node::add_child(&root, child.clone());

        assert_eq!(root.borrow().children.len(), 1);
        assert!(child.borrow().parent.is_some());
        assert!(root.borrow().is_root());
        assert!(!child.borrow().is_root());
    }
}
