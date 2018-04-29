pub trait Operable {
    type Key: PartialEq + PartialOrd;
    type Val;

    fn lookup(&self, key: &Self::Key) -> Option<&Self::Val>;
    fn insert(self, key: Self::Key, val: Self::Val) -> Self;
    fn remove(self, key: &Self::Key) -> Self;
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

use BinaryTree::{Leaf, NonLeaf};

impl<K, V> BinaryTree<K, V> {
    fn remove_smallest(self) -> Option<(K, V, Self)> {
        match self {
            Leaf => None,
            NonLeaf { key, val, left, right } => {
                match left.remove_smallest() {
                    None => Some((key, val, *right)),
                    Some((k, v, tree)) => Some(
                        (k, v, NonLeaf { key, val, left: Box::new(tree), right })
                    )
                }
            }
        }
    }
}

impl<K: PartialEq + PartialOrd, V> Operable for BinaryTree<K, V> {
    type Key = K;
    type Val = V;

    fn lookup(&self, key: &Self::Key) -> Option<&Self::Val> {
        match *self {
            Leaf => None,
            NonLeaf { key: ref k, val: ref v, left:  _, right:  _ }
                if k == key => Some(&v),
            NonLeaf { key: ref k, val:     _, left:  _, ref right }
                if k < key => right.lookup(key),
            NonLeaf { key: ref k, val:     _, ref left, right:  _ }
                if k > key => left.lookup(key),
            NonLeaf { .. } => None
        }
    }

    fn insert(self, key: Self::Key, val: Self::Val) -> Self {
        match self {
            Leaf => NonLeaf { key, val, left: Box::new(Leaf), right: Box::new(Leaf) },
            NonLeaf { key: k, val: v, left, right } => {
                if k == key {
                    NonLeaf { key, val, left, right }
                } else if k > key {
                    NonLeaf { key: k, val: v, left: Box::new(left.insert(key, val)), right }
                } else {
                    NonLeaf { key: k, val: v, left, right: Box::new(right.insert(key, val)) }
                }
            }
        }
    }

    fn remove(self, key: &Self::Key) -> Self {
        match self {
            Leaf => self,
            NonLeaf { key: k, val, left, right } => {
                if k == *key {
                    if let Leaf = *left { *right }
                    else if let Leaf = *right { *left }
                    else {
                        let (key, val, right) = right.remove_smallest().unwrap();
                        NonLeaf { key, val, left, right: Box::new(right) }
                    }
                } else if k > *key {
                    NonLeaf { key: k, val, left: Box::new(left.remove(key)), right }
                } else {
                    NonLeaf { key: k, val, left, right: Box::new(right.remove(key)) }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_key_lookup() {
        let tree = NonLeaf {
            key: 5,
            val: "Hello",
            left: Box::new(NonLeaf {
                key: 1,
                val: "World",
                left: Box::new(Leaf),
                right: Box::new(NonLeaf {
                    key: 3,
                    val: "Sucker",
                    left: Box::new(Leaf),
                    right: Box::new(Leaf)
                })
            }),
            right: Box::new(Leaf)
        };

        assert_eq!(*tree.lookup(&3).unwrap(), "Sucker");
    }

    #[test]
    fn char_key_lookup() {
        let tree = NonLeaf {
            key: 'G',
            val: "Hello",
            left: Box::new(NonLeaf {
                key: 'A',
                val: "World",
                left: Box::new(Leaf),
                right: Box::new(NonLeaf {
                    key: 'E',
                    val: "Sucker",
                    left: Box::new(Leaf),
                    right: Box::new(Leaf)
                })
            }),
            right: Box::new(Leaf)
        };

        assert_eq!(*tree.lookup(&'E').unwrap(), "Sucker");
    }

    #[test]
    fn insert_new_key() {
        let tree = NonLeaf {
            key: 5,
            val: "Hello",
            left: Box::new(NonLeaf {
                key: 1,
                val: "World",
                left: Box::new(Leaf),
                right: Box::new(NonLeaf {
                    key: 3,
                    val: "Sucker",
                    left: Box::new(Leaf),
                    right: Box::new(Leaf)
                })
            }),
            right: Box::new(Leaf)
        };

        assert_eq!(*tree.insert(0, "Ahoo!").lookup(&0).unwrap(), "Ahoo!");
    }

    #[test]
    fn insert_existing_key() {
        let tree = NonLeaf {
            key: 5,
            val: "Hello",
            left: Box::new(NonLeaf {
                key: 1,
                val: "World",
                left: Box::new(Leaf),
                right: Box::new(NonLeaf {
                    key: 3,
                    val: "Sucker",
                    left: Box::new(Leaf),
                    right: Box::new(Leaf)
                })
            }),
            right: Box::new(Leaf)
        };

        assert_eq!(*tree.lookup(&1).unwrap(), "World");
        assert_eq!(*tree.insert(1, "Ahoo!").lookup(&1).unwrap(), "Ahoo!");
    }

    #[test]
    fn remove_existing_key() {
        let tree = NonLeaf {
            key: 5,
            val: "Hello",
            left: Box::new(NonLeaf {
                key: 1,
                val: "World",
                left: Box::new(NonLeaf {
                    key: 0,
                    val: "Magic",
                    left: Box::new(Leaf),
                    right: Box::new(Leaf)
                }),
                right: Box::new(NonLeaf {
                    key: 3,
                    val: "Sucker",
                    left: Box::new(NonLeaf {
                        key: 2,
                        val: "Auch",
                        left: Box::new(Leaf),
                        right: Box::new(Leaf)
                    }),
                    right: Box::new(Leaf)
                })
            }),
            right: Box::new(Leaf)
        };

        assert_eq!(*tree.lookup(&1).unwrap(), "World");
        assert_eq!(tree.remove(&1).lookup(&1), None);
    }

    #[test]
    fn remove_non_existent_key() {
        let tree = NonLeaf {
            key: 5,
            val: "Hello",
            left: Box::new(NonLeaf {
                key: 1,
                val: "World",
                left: Box::new(NonLeaf {
                    key: 0,
                    val: "Magic",
                    left: Box::new(Leaf),
                    right: Box::new(Leaf)
                }),
                right: Box::new(NonLeaf {
                    key: 3,
                    val: "Sucker",
                    left: Box::new(NonLeaf {
                        key: 2,
                        val: "Auch",
                        left: Box::new(Leaf),
                        right: Box::new(Leaf)
                    }),
                    right: Box::new(Leaf)
                })
            }),
            right: Box::new(Leaf)
        };

        assert_eq!(tree.remove(&4).lookup(&4), None);
    }
}
