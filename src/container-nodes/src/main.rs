#![allow(dead_code)]

fn main() {
    let mut container = Container::new();
    let node0 = container.add_node("node0");
    let node1 = container.add_node("node1");
    let node2 = container.add_node("node2");
    let node3 = container.add_node("node3");
}

struct Container {
    nodes: Vec<Node>,
}

struct Node {
    name: String,
    index: usize,
}

struct NodeSetRef<'a> {
    container: &'a mut Container,
    index: usize,
}

impl<'a> NodeSetRef<'a> {
    fn find_left_neighbor(&mut self) -> Option<NodeSetRef<'_>> {
        self.container.get_node_ref(self.index - 1)
    }
    fn find_right_neighbor(&mut self) -> Option<NodeSetRef<'_>> {
        self.container.get_node_ref(self.index + 1)
    }
}

impl Container {
    fn new() -> Container {
        Container { nodes: vec![] }
    }

    fn add_node<'a, S>(&'a mut self, name: S) -> NodeSetRef<'a>
    where
        S: Into<String>,
    {
        let index = self.nodes.len();
        let node = Node {
            name: name.into(),
            index: index,
        };
        self.nodes.push(node);
        NodeSetRef {
            container: self,
            index,
        }
    }

    fn get_node_ref(&mut self, index: usize) -> Option<NodeSetRef<'_>> {
        let length = self.nodes.len();
        if length <= index {
            return None;
        }
        Some(NodeSetRef {
            container: self,
            index: index,
        })
    }
}
