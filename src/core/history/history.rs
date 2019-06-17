use crate::core::graph::graph::*;
use crate::core::history::hashlist::*;
use crate::core::history::history::Ref::*;
use core::fmt::Debug;
use core::hash::Hash;
use std::collections::HashMap;
use std::rc::Rc;

type Hashs = Rc<HashList>;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum Ref {
    Tag(Hashs, String),
    Branch(Hashs, String),
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Author(pub String);

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Comment(pub String);

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Commit<Item>
where
    Item: PartialEq + Eq + Clone + Hash + Debug,
{
    author: Author,
    comment: Comment,
    hash: NodeHash,
    item: Item,
}

// Trait for a Hasher, in charge of hashing an Item for a repository
trait Hasher<Item>
where
    Item: PartialEq + Eq + Clone + Hash + Debug,
{
    fn hash(item: &Item, previous: Option<NodeHash>) -> NodeHash;
}

/// Git-like Repository for an item
/// FIXME: it should be a trait, with an in-memory, and persistent storage implementations for instance
pub struct Repository<Item>
where
    Item: PartialEq + Eq + Clone + Hash + Debug,
{
    current: Ref,
    // FIXME: we need the hasher to be stored
    // hasher: &Hasher<Item>
    refs: Vec<Ref>,
    commits: HashMap<NodeHash, Commit<Item>>,
}

impl Ref {
    pub fn is_empty(&self) -> bool {
        self.hashs().is_empty()
    }

    pub fn name(&self) -> &str {
        match self {
            Tag(_, n) => n,
            Branch(_, n) => n,
        }
    }

    pub fn hashs(&self) -> Hashs {
        match self {
            Tag(hashs, _) => hashs.clone(),
            Branch(hashs, _) => hashs.clone(),
        }
    }

    pub fn is_read_only(&self) -> bool {
        match self {
            Tag(_, _) => true,
            _ => false,
        }
    }
}

impl<Item> Repository<Item>
where
    Item: PartialEq + Eq + Clone + Hash + Debug,
{
    /// Creates a new Repository
    pub fn new<I>() -> Repository<I>
    where
        I: PartialEq + Eq + Clone + Hash + Debug,
    {
        let master = Branch(HashList::empty(), "master".to_string());
        Repository {
            current: master.clone(),
            refs: vec![master],
            commits: HashMap::new(),
        }
    }

    /// Creates and add a new commit to the current branch
    pub fn commit(&mut self, item: Item, author: Author, comment: Comment) -> Result<Ref, String> {
        if self.current.is_read_only() {
            Err(format!("Cannot modify Reference {}", self.current.name()))
        } else {
            let commit = self.create_commit(item, author, comment);
            let new_head = HashList::cons(commit.hash, self.current.hashs());
            // Updating repo
            self.commits.insert(commit.hash, commit);
            self.current = Branch(new_head.clone(), self.current.name().to_string());
            Ok(self.current.clone())
        }
    }

    fn create_commit(&self, item: Item, author: Author, comment: Comment) -> Commit<Item> {
        let last_hash = self.current.hashs().head_option();
        let commit_hash = Repository::compute_hash(&item, last_hash);
        Commit {
            author: author,
            comment: comment,
            hash: commit_hash,
            item: item,
        }
    }

    // Branch / Tags

    pub fn tag(&mut self, name: String) -> Ref {
        let tag = Tag(self.current.hashs(), name);
        self.refs.push(tag.clone());
        tag
    }

    pub fn branch(&mut self, name: String) -> Ref {
        let branch = Branch(self.current.hashs(), name);
        self.refs.push(branch.clone());
        branch
    }

    // Checkout

    pub fn checkout_tag(&mut self, _name: String) -> Ref {
        unimplemented!()
    }

    pub fn checkout_branch(&mut self, _name: String) -> Ref {
        unimplemented!()
    }

    pub fn checkout_hash(&mut self, _hash: NodeHash) -> Ref {
        unimplemented!()
    }

    // Reset

    pub fn reset_soft(&mut self, _hash: NodeHash) -> (Ref, Vec<GraphCommand>) {
        unimplemented!()
    }

    pub fn reset_hard(&mut self, _hash: NodeHash) -> Ref {
        unimplemented!()
    }

    // Rebasing

    pub fn rebase(&mut self, _hash: NodeHash, _onto: NodeHash) -> Ref {
        unimplemented!()
    }

    /// Compute a hash for the given command, inclusing the previous hash
    fn compute_hash(_command: &Item, _last_hash: Option<NodeHash>) -> NodeHash {
        // FIXME this call should be replaced with a call to the hasher
        unimplemented!()
    }
}
