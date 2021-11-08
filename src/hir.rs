use std::{collections::HashMap, sync::Arc};

pub struct Hir {
    nodes: HashMap<NodeId, Arc<Node>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(u64);

pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
}

pub enum NodeKind {
    Function(Function),
}

pub struct Function {

}

pub struct Module {
    items: Vec<NodeId>,
}

impl Module {
    pub fn empty() -> Module {
        Module {
            items: Vec::new(),
        }
    }
}

pub struct Global {

}

impl Global {
    fn type_identifier(&self) -> &str {
        "i32"
    }

    /// An identifier that is globally unique.
    fn unique_identifier(&self) -> &str {
        "xxx"
    }
}
