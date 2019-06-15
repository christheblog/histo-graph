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
        Rc::new(HashList::Node {
            hash: hash,
            tail: Rc::new(HashList::Nil),
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
        match (l1.as_ref(), l2.as_ref()) {
            (HashList::Nil, _) | (_, HashList::Nil) => None,
            (HashList::Node { hash: x, .. }, HashList::Node { hash: y, .. }) if x == y => Some(*x),
            (HashList::Node { hash: _, tail: xs }, _) => HashList::first_common(xs.clone(), l2),
        }
    }

    /// Rebasing a list onto another one.
    pub fn rebase(list: Rc<HashList>, onto: Rc<HashList>) -> Option<Rc<HashList>> {
        HashList::first_common(list.clone(), onto.clone()).map(|node| unimplemented!())
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
        match self {
            HashList::Node { hash: x, .. } => Some(*x),
            HashList::Nil => None,
        }
    }

    pub fn tail(&self) -> Rc<HashList> {
        match self.tail_option() {
            Some(x) => x,
            None => panic!["Cannot take the tail of an empty list"],
        }
    }

    pub fn tail_option(&self) -> Option<Rc<HashList>> {
        match self {
            HashList::Node { tail: xs, .. } => Some(xs.clone()),
            HashList::Nil => None,
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
        match self {
            HashList::Node {
                hash: x, tail: xs, ..
            } if predicate(x) => Rc::new(HashList::Node {
                hash: *x,
                tail: xs.take_while(predicate),
            }),
            _ => Rc::new(HashList::Nil),
        }
    }
}
