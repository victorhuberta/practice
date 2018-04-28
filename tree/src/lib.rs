pub trait Operable {
    type Key: PartialEq + PartialOrd;
    type Val;

    fn lookup(&self, key: &Self::Key) -> Option<&Self::Val>;
    //fn insert(self, key: Self::Key, val: Self::Val) -> Self;
    //fn remove(self, key: Self::Key) -> Self;
}

pub enum BinaryTree<K, V> {
    Leaf,
    NonLeaf {
        key: K,
        val: V,
        left: Box<BinaryTree<K, V>>,
        right: Box<BinaryTree<K, V>>
    }
}

impl<K: PartialEq + PartialOrd, V> Operable for BinaryTree<K, V> {
    type Key = K;
    type Val = V;

    fn lookup(&self, key: &Self::Key) -> Option<&Self::Val> {
        match *self {
            BinaryTree::Leaf => None,
            BinaryTree::NonLeaf { key: ref k, val: ref v, left:  _, right:  _ } if k == key => Some(&v),
            BinaryTree::NonLeaf { key: ref k, val:     _, left:  _, ref right } if k < key => right.lookup(key),
            BinaryTree::NonLeaf { key: ref k, val:     _, ref left, right:  _ } if k > key => left.lookup(key),
            BinaryTree::NonLeaf { .. } => None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_key_lookup() {
        let tree = BinaryTree::NonLeaf {
            key: 5,
            val: "Hello",
            left: Box::new(BinaryTree::NonLeaf {
                key: 1,
                val: "World",
                left: Box::new(BinaryTree::Leaf),
                right: Box::new(BinaryTree::NonLeaf {
                    key: 3,
                    val: "Sucker",
                    left: Box::new(BinaryTree::Leaf),
                    right: Box::new(BinaryTree::Leaf)
                })
            }),
            right: Box::new(BinaryTree::Leaf)
        };

        assert_eq!(*tree.lookup(&3).unwrap(), "Sucker");
    }

    #[test]
    fn char_key_lookup() {
        let tree = BinaryTree::NonLeaf {
            key: 'G',
            val: "Hello",
            left: Box::new(BinaryTree::NonLeaf {
                key: 'A',
                val: "World",
                left: Box::new(BinaryTree::Leaf),
                right: Box::new(BinaryTree::NonLeaf {
                    key: 'E',
                    val: "Sucker",
                    left: Box::new(BinaryTree::Leaf),
                    right: Box::new(BinaryTree::Leaf)
                })
            }),
            right: Box::new(BinaryTree::Leaf)
        };

        assert_eq!(*tree.lookup(&'E').unwrap(), "Sucker");
    }
}
