fn max_depth(root: Option<&TreeNode>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let left_depth = max_depth(root.unwrap().left.as_ref());
    let right_depth = max_depth(root.unwrap().right.as_ref());

    // Add 1 to account for the current node
    std::cmp::max(left_depth, right_depth) + 1
}

// Define the TreeNode struct if needed
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}