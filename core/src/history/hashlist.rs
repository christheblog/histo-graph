use std::rc::Rc;

// Node hash
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct NodeHash(u64);

// HashList
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum HashList {
    Nil,
    Node { hash: NodeHash, tail: Rc<HashList> },
}

impl HashList {
    /// creates an empty list
    pub fn empty() -> Rc<HashList> {
        Rc::new(HashList::Nil)
    }

    /// creates a singleton list
    pub fn singleton(hash: NodeHash) -> Rc<HashList> {
        use HashList::*;
        Rc::new(Node {
            hash: hash,
            tail: Rc::new(Nil),
        })
    }

    pub fn cons(hash: NodeHash, list: Rc<HashList>) -> Rc<HashList> {
        Rc::new(HashList::Node {
            hash: hash,
            tail: list.clone(),
        })
    }

    /// Find the first common NodeHash between 2 HashLists
    pub fn first_common(l1: Rc<HashList>, l2: Rc<HashList>) -> Option<NodeHash> {
        use HashList::*;
        match (l1.as_ref(), l2.as_ref()) {
            (Nil, _) | (_, Nil) => None,
            (Node { hash: x, .. }, Node { hash: y, .. }) if x == y => Some(*x),
            (Node { hash: _, tail: xs }, _) => HashList::first_common(xs.clone(), l2),
        }
    }

    /// Rebasing a list onto another one.
    pub fn rebase(list: Rc<HashList>, onto: Rc<HashList>) -> Option<Rc<HashList>> {
        HashList::first_common(list.clone(), onto.clone()).map(|_node| unimplemented!())
    }

    // List functions

    pub fn is_empty(&self) -> bool {
        match self {
            HashList::Nil => true,
            _ => false,
        }
    }

    pub fn head(&self) -> NodeHash {
        match self.head_option() {
            Some(x) => x,
            None => panic!["Cannot take the head of an empty list"],
        }
    }

    pub fn head_option(&self) -> Option<NodeHash> {
        use HashList::*;
        match self {
            Node { hash: x, .. } => Some(*x),
            Nil => None,
        }
    }

    pub fn tail(&self) -> Rc<HashList> {
        match self.tail_option() {
            Some(x) => x,
            None => panic!["Cannot take the tail of an empty list"],
        }
    }

    pub fn tail_option(&self) -> Option<Rc<HashList>> {
        use HashList::*;
        match self {
            Node { tail: xs, .. } => Some(xs.clone()),
            Nil => None,
        }
    }

    pub fn contains<P>(&self, predicate: P) -> bool
    where
        P: Fn(&NodeHash) -> bool,
    {
        match self.head_option() {
            Some(x) if predicate(&x) => true,
            Some(_) => self.tail().contains(predicate),
            None => false,
        }
    }

    pub fn take_while<P>(&self, predicate: P) -> Rc<HashList>
    where
        P: Fn(&NodeHash) -> bool,
    {
        use HashList::*;
        match self {
            Node {
                hash: x, tail: xs, ..
            } if predicate(x) => Rc::new(Node {
                hash: *x,
                tail: xs.take_while(predicate),
            }),
            _ => Rc::new(Nil),
        }
    }

    pub fn skip_while<P>(&self, predicate: P) -> Rc<HashList>
    where
        P: Fn(&NodeHash) -> bool,
    {
        use HashList::*;
        match self {
            Node {
                hash: x, tail: xs, ..
            } => {
                if predicate(x) {
                    xs.take_while(predicate)
                } else {
                    xs.clone()
                }
            }
            _ => Rc::new(Nil),
        }
    }

    // Iterator

    pub fn iter(&self) -> HashListIter {
        HashListIter {
            current: Rc::new(self.clone()),
        }
    }
}

// Hashlist iterator
pub struct HashListIter {
    current: Rc<HashList>,
}

impl Iterator for HashListIter {
    type Item = NodeHash;
    fn next(&mut self) -> Option<Self::Item> {
        let nxt = self.current.head_option();
        self.current = self.current.tail().clone();
        nxt
    }
}
