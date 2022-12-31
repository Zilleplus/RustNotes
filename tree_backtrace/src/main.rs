#[derive(Debug)]
enum Tree {
    Branch(BranchData),
    Leaf(LeafData), // end point
}

fn points(t: &Tree) -> i32 {
    match t {
        Tree::Branch(node_data) => node_data.points,
        Tree::Leaf(leaf_data) => leaf_data.points,
    }
}

#[derive(Debug)]
struct LeafData {
    points: i32,
}

impl LeafData {
    fn new_tree(points: i32) -> Tree {
        Tree::Leaf(LeafData { points })
    }
}

#[derive(Debug)]
struct BranchData {
    points: i32,
    subtree: Vec<Tree>,
}

impl BranchData {
    fn new_tree(subtree: Option<Vec<Tree>>) -> Tree {
        let total_points = subtree
            .as_ref()
            .map(|x| x.iter().map(points).sum())
            .unwrap_or(0);

        Tree::Branch(BranchData {
            points: total_points,
            subtree: subtree.unwrap_or(Vec::new()),
        })
    }
}

fn main() {
    println!("Demo tree");
    println!("The branches points contains the sum of the leafs points.");

    let subtree = BranchData::new_tree(Some(vec![LeafData::new_tree(2), LeafData::new_tree(3)]));
    let root = BranchData::new_tree(Some(vec![subtree]));

    println!("Tree: {:?}", &root);
}
