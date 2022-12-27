use std::{borrow::BorrowMut, rc::Rc};

#[derive(Debug)]
struct NodeData {
    subtree: Vec<Rc<Tree>>,
}

impl NodeData {
    fn new() -> NodeData {
        NodeData {
            subtree: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct LeafData {
    name: String,
}

#[derive(Debug)]
enum Tree {
    Node(NodeData),
    Leaf(LeafData),
}

fn all_elements(t: &Tree) -> Vec<Rc<Tree>> {
    let mut elements = Vec::new();

    match t {
        Tree::Node(node_data) => {
            for e in node_data.subtree.iter().map(|x| x.clone()) {
                elements.push(e);
            }
        }
        _ => {}
    }

    elements
}

fn mutate_node(n: &mut Tree) {
    *n = Tree::Leaf(LeafData {
        name: "".to_owned(),
    });
}

fn main() {
    let mut root = Rc::new(Tree::Node(NodeData::new()));
    // get_mut: Returns a mutable reference into the given Rc, if there are no other Rc or Weak pointers to the same allocation.
    // so it works here, but won't work once there is more then one RC object...
    // >
    // > mutate_node(Rc::get_mut(&mut root).unwrap());
    // >
    //
    // If we want to mutate a shared object we need unsafe code. No way around this, more then 1
    // reader en a write action is forbidden. The get_mut will simple return None.

    let first_node = Rc::new(Tree::Node(NodeData::new()));

    // This works as we only have one refence on the root.
    match Rc::get_mut(&mut root).unwrap() {
        Tree::Node(node_data) => {
            node_data.subtree.push(first_node.clone());
        }
        _ => {
            panic!("Root should not be a leaf.");
        }
    };

    println!("The tree is {:?}", root);

    // Now we start to keep track of all the nodes in a list. So the nodes will have more then 1
    // owner. (hence the reason we want to use Rc in the first place)
    let elements = vec!(root.clone(), first_node.clone());

    // If we try to get a mutable reference now, we are in trouble:
    match Rc::get_mut(&mut root) {
        Some(_) =>  panic!("Mutable to ref to Rc with two onwers."),
        None => println!("SUCCESS: unable to mutate a shared Rc.")
    };
}
