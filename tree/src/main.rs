#[derive(Debug, Clone)]
struct Point(u32, u32);

impl Point {
    fn equal(&self, p: &Point) -> bool {
        self.0.eq(&p.0) && self.1.eq(&p.1)
    }
}

#[derive(Debug, Default)]
struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    fn new() -> Tree {
        Tree { nodes: Vec::new() }
    }
    fn add_children(&mut self, parent: Point, child: Point) -> usize {
        let parent_index = self.get_node(&parent).unwrap_or_else(|| {
            panic!("Parent node not found");
        });
        let child_index = self.node(child);

        self.nodes[child_index].parent = Some(parent_index);
        self.nodes[parent_index].children.push(child_index);

        child_index
    }
    fn get_node(&self, value: &Point) -> Option<usize> {
        for node in &self.nodes {
            if node.value.equal(&value) {
                return Some(node.idx);
            }
        }
        None
    }
    fn node(&mut self, value: Point) -> usize {
        //first see if it exists
        match self.get_node(&value) {
            Some(index) => index,
            None => {
                // Otherwise, add new node
                let idx = self.nodes.len();
                self.nodes.push(Node::new(idx, value));
                idx
            }
        }
    }
}

#[derive(Debug)]
struct Node {
    idx: usize,
    value: Point,
    parent: Option<usize>,
    children: Vec<usize>,
    visited: bool,
}

impl Node {
    fn new(idx: usize, value: Point) -> Self {
        Node {
            idx,
            value,
            parent: None,
            children: vec![],
            visited: false,
        }
    }
}
fn main() {
    let mut tree = Tree::new();

    tree.node(Point(1, 0));

    tree.add_children(Point(1, 0), Point(2, 0));
    tree.add_children(Point(1, 0), Point(3, 0));
    tree.add_children(Point(2, 0), Point(4, 0));

    println!("tree : {:?}", tree);
    println!("search tree : {:?}", tree.get_node(&Point(3, 0)).unwrap());
}
