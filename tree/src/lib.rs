use std::rc::Rc;

pub trait Operable {
    type Key: PartialEq + PartialOrd;
    type Val;

    fn lookup(&self, key: &Self::Key) -> Option<&Self::Val>;
    fn insert(self, key: Self::Key, val: Self::Val) -> Self;
    fn remove(self, key: &Self::Key) -> Self;
    fn map_dfs<F: Fn(Self::Val) -> Self::Val>(self, f: Rc<F>) -> Self;
    fn map_bfs<F: Fn(Self::Val) -> Self::Val>(self, f: Rc<F>) -> Self;
}

#[derive(Debug, Clone, PartialEq)]
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

    fn map_dfs<F: Fn(Self::Val) -> Self::Val>(self, f: Rc<F>) -> Self {
        match self {
            Leaf => self,
            NonLeaf { key, val, left, right } => NonLeaf {
                key,
                val: f(val),
                left: Box::new(left.map_dfs(f.clone())),
                right: Box::new(right.map_dfs(f.clone()))
            }
        }
    }

    fn map_bfs<F: Fn(Self::Val) -> Self::Val>(self, q: [Self], f: Rc<F>) -> Self {
        if q.len() == 0 && let NonLeaf { .. } = self {
            q.push(self);
        } else if let Leaf = self {
            self
        }

        if q.len() > 0 {
            let front = q.remove(0);
            match front {
                Leaf => front,
                NonLeaf { key, val, left, right } => {
                    q.push(left);
                    q.push(right);
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

        assert_eq!(tree.clone().remove(&4), tree);
    }

    #[test]
    fn double_tree_vals_with_map_dfs() {
        let tree = NonLeaf {
            key: 5,
            val: 2,
            left: Box::new(NonLeaf {
                key: 1,
                val: 3,
                left: Box::new(Leaf),
                right: Box::new(NonLeaf {
                    key: 3,
                    val: 4,
                    left: Box::new(Leaf),
                    right: Box::new(Leaf)
                })
            }),
            right: Box::new(Leaf)
        };
        let expected = NonLeaf {
            key: 5,
            val: 4,
            left: Box::new(NonLeaf {
                key: 1,
                val: 6,
                left: Box::new(Leaf),
                right: Box::new(NonLeaf {
                    key: 3,
                    val: 8,
                    left: Box::new(Leaf),
                    right: Box::new(Leaf)
                })
            }),
            right: Box::new(Leaf)
        };

        assert_eq!(tree.map_dfs(Rc::new(|x| x*2)), expected);
    }

    #[test]
    fn triple_tree_vals_with_map_bfs() {
        let tree = NonLeaf {
            key: 5,
            val: 2,
            left: Box::new(NonLeaf {
                key: 1,
                val: 3,
                left: Box::new(Leaf),
                right: Box::new(NonLeaf {
                    key: 3,
                    val: 4,
                    left: Box::new(Leaf),
                    right: Box::new(Leaf)
                })
            }),
            right: Box::new(Leaf)
        };
        let expected = NonLeaf {
            key: 5,
            val: 6,
            left: Box::new(NonLeaf {
                key: 1,
                val: 9,
                left: Box::new(Leaf),
                right: Box::new(NonLeaf {
                    key: 3,
                    val: 12,
                    left: Box::new(Leaf),
                    right: Box::new(Leaf)
                })
            }),
            right: Box::new(Leaf)
        };

        assert_eq!(tree.map_bfs(Rc::new(|x| x*3)), expected);
    }
}
