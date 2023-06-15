#[derive(Debug, Default, PartialEq)]
pub struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

/// Binary search tree
#[derive(Debug, Default)]
pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> BinarySearchTree<T>
where
    T: std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone,
{
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn sort(&self) -> Vec<T> {
        let mut result = Vec::new();
        self.inorder(self.root.as_ref(), &mut result);
        result
    }

    pub fn inorder_tree_walk(&self) -> Vec<T> {
        let mut result = Vec::new();
        self.inorder(self.root.as_ref(), &mut result);
        result
    }

    pub fn tree_search(&self, data: T) -> Option<&Box<Node<T>>> {
        let mut node = self.root.as_ref();
        while let Some(n) = node {
            if n.data == data {
                return node;
            } else if n.data > data {
                node = n.left.as_ref();
            } else {
                node = n.right.as_ref();
            }
        }
        None
    }

    pub fn interative_tree_search(&self, data: T) -> Option<&Box<Node<T>>> {
        let mut node = self.root.as_ref();
        while let Some(n) = node {
            if n.data == data {
                return node;
            } else if n.data > data {
                node = n.left.as_ref();
            } else {
                node = n.right.as_ref();
            }
        }
        None
    }

    pub fn tree_minmum(&self) -> Option<&Box<Node<T>>> {
        let mut node = self.root.as_ref();
        while let Some(n) = node {
            if n.left.is_none() {
                return node;
            } else {
                node = n.left.as_ref();
            }
        }
        None
    }

    pub fn tree_maxmum(&self) -> Option<&Box<Node<T>>> {
        let mut node = self.root.as_ref();
        while let Some(n) = node {
            if n.right.is_none() {
                return node;
            } else {
                node = n.right.as_ref();
            }
        }
        None
    }

    pub fn tree_search_with_path(&self, data: T) -> Option<(Vec<&Node<T>>, &Node<T>)> {
        let mut path = Vec::new();
        let mut current = &self.root;
        while let Some(node) = current {
            if data < node.data {
                current = &node.left;
            } else if data > node.data {
                current = &node.right;
            } else {
                return Some((path, node));
            }
            path.push(node);
        }
        None
    }

    pub fn tree_successor(&self, data: T) -> Option<&Node<T>> {
        let (mut path, mut node) = self.tree_search_with_path(data)?;
        if let Some(right) = &node.right {
            let mut node = right;
            while let Some(left) = &node.left {
                node = left;
            }
            return Some(node);
        }
        while let Some(parent) = path.pop() {
            if parent.right.as_deref() != Some(node) {
                return Some(parent);
            }
            node = parent;
        }
        None
    }

    pub fn inorder(&self, node: Option<&Box<Node<T>>>, result: &mut Vec<T>) {
        if let Some(node) = node {
            self.inorder(node.left.as_ref(), result);
            result.push(node.data.clone());
            self.inorder(node.right.as_ref(), result);
        }
    }

    pub fn contains(&self, data: T) -> bool {
        let mut node = self.root.as_ref();
        while let Some(n) = node {
            if n.data == data {
                return true;
            } else if n.data > data {
                node = n.left.as_ref();
            } else {
                node = n.right.as_ref();
            }
        }
        false
    }

    pub fn tree_insert(&mut self, data: T) {
        let mut node = &mut self.root;
        while let Some(n) = node {
            if n.data > data {
                node = &mut n.left;
            } else {
                node = &mut n.right;
            }
        }
        *node = Some(Box::new(Node {
            data,
            left: None,
            right: None,
        }));
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_search_binary_tree() {
        use super::BinarySearchTree;
        let mut tree = BinarySearchTree::new();
        tree.tree_insert(5);
        tree.tree_insert(3);
        tree.tree_insert(7);
        tree.tree_insert(2);

        let ret = tree.sort();
        println!("ret = {:?}", ret);

        let min = tree.tree_minmum();
        println!("min = {:?}", min);
        let max = tree.tree_maxmum();
        println!("max = {:?}", max);
    }
}
