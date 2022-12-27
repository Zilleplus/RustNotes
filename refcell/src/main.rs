// The cell libraries RefCell allows only 1 owner.
// You can only have a bunch of readers or 1 writer.
// It uses a counter on both Ref and RefMut to check
// this at compile time.
// -> std::cell this when you can't convince the borrow checker
// that everything is ok. Eventhough in theory everything is ok.
use std::{
    borrow::BorrowMut,
    cell::{
        Ref,     // read only reference
        RefCell, // owner of the data
        RefMut,  // mutable reference
    },
};

#[derive(Debug)]
struct NodeData {
    subtree: Vec<RefCell<Tree>>,
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

fn print_immutable_tree(t: &Tree)
{
    println!("The immutable ref tree is {:?}", t);
}

fn main() {
    let mut root = RefCell::new(Tree::Node(NodeData::new()));
    let first_node = RefCell::new(Tree::Node(NodeData::new()));

    match root.get_mut() {
        Tree::Node(node_data) => {
            node_data.subtree.push(first_node);
        }
        _ => {
            panic!("Root should not be a leaf.");
        }
    };

    println!("The tree is {:?}", root);
    let immutable_ref: Ref<Tree> = root.borrow();
    print_immutable_tree(&immutable_ref); // <-- notice the '&' to get to reference type.
    // The automatic referencing/dereferencing makes it able so we can use it like normal ref.
    let tree: &Tree = &immutable_ref; // What we get if we specifiy normal reference.
    let tree_rewf: &Ref<Tree> = &immutable_ref; // What we get without type specifier.
    // Till now everyting ok:

    // This is not ok anymore:
    // >
    // >  let mutable_ref: RefMut<Tree> = root.borrow_mut();
    // >
    // If we were to do this, the program would panic.
}
