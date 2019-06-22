use std::collections::HashMap;
use std::rc::Rc;

use core::fmt::Debug;
use core::hash::Hash;

use crate::core::history::hashlist::*;
use crate::core::history::history::Ref::*;

type Hashs = Rc<HashList>;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum Ref {
    Detached(Hashs),
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
pub trait Hasher<Item>
where
    Item: PartialEq + Eq + Clone + Hash + Debug,
{
    fn hash(&self, item: &Item, previous: Option<NodeHash>) -> NodeHash;
}

/// Git-like Repository for an item
/// FIXME: it should be a trait, with an in-memory, and persistent storage implementations for instance
pub struct Repository<Item, ItemHasher: Hasher<Item>>
where
    Item: PartialEq + Eq + Clone + Hash + Debug,
{
    current: Ref,
    hasher: ItemHasher,
    refs: Vec<Ref>,
    commits: HashMap<NodeHash, Commit<Item>>,
}

impl Ref {
    pub fn is_empty(&self) -> bool {
        self.hashs().is_empty()
    }

    pub fn name(&self) -> &str {
        match self {
            Detached(_) => "Detached HEAD",
            Tag(_, n) => n,
            Branch(_, n) => n,
        }
    }

    pub fn hashs(&self) -> Hashs {
        match self {
            Detached(hashs) => hashs.clone(),
            Tag(hashs, _) => hashs.clone(),
            Branch(hashs, _) => hashs.clone(),
        }
    }

    pub fn is_read_only(&self) -> bool {
        match self {
            Tag(_, _) | Detached(_) => true,
            _ => false,
        }
    }
}

impl<RepoItem, ItemHasher> Repository<RepoItem, ItemHasher>
where
    RepoItem: PartialEq + Eq + Clone + Hash + Debug,
    ItemHasher: Hasher<RepoItem>,
{
    /// Creates a new Repository
    pub fn new<I, H>(hasher: H) -> Repository<I, H>
    where
        I: PartialEq + Eq + Clone + Hash + Debug,
        H: Hasher<I>,
    {
        let master = Branch(HashList::empty(), "master".to_string());
        Repository {
            current: master.clone(),
            hasher: hasher,
            refs: vec![master],
            commits: HashMap::new(),
        }
    }

    /// Creates and add a new commit to the current branch
    /// returns an error if current selection is not a branch
    pub fn commit(
        &mut self,
        item: RepoItem,
        author: Author,
        comment: Comment,
    ) -> Result<Ref, String> {
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

    fn create_commit(&self, item: RepoItem, author: Author, comment: Comment) -> Commit<RepoItem> {
        let last_hash = self.current.hashs().head_option();
        let commit_hash = self.hasher.hash(&item, last_hash);
        Commit {
            author: author,
            comment: comment,
            hash: commit_hash,
            item: item,
        }
    }

    /// Iterates through the commits from the current ref in order
    /// From the most recent to the oldest
    // FIXME Iterator here !!!
    pub fn commits(&self) -> Vec<&Commit<RepoItem>> {
        self.current
            .hashs()
            .iter()
            .map(|x| self.find_commit(x).unwrap()) // FIXME self doesn't live long enough ?
            .collect()
    }

    // Branch / Tags

    /// Tag the current commit with the given name
    pub fn tag(&mut self, name: &str) -> Ref {
        let tag = Tag(self.current.hashs(), name.to_string());
        self.refs.push(tag.clone());
        tag
    }

    pub fn branch(&mut self, name: &str) -> Ref {
        let branch = Branch(self.current.hashs(), name.to_string());
        self.refs.push(branch.clone());
        branch
    }

    // Checkout

    pub fn checkout_tag(&mut self, name: &str) -> Result<Ref, String> {
        match self.find_tag(&name) {
            None => Err(format!("Tag {} doesn't exists", name)),
            Some(t) => {
                self.current = t.clone();
                Ok(t.clone())
            }
        }
    }

    pub fn checkout_branch(&mut self, name: &str) -> Result<Ref, String> {
        match self.find_branch(&name) {
            None => Err(format!("Branch {} doesn't exists", name)),
            Some(t) => {
                self.current = t.clone();
                Ok(t.clone())
            }
        }
    }

    pub fn checkout_hash(&mut self, hash: NodeHash) -> Result<Ref, String> {
        let hashs = self.find_hashes_from(&self.current, hash);
        match hashs {
            None => Err(format!(
                "Hash {:?} couldn't be found on the current Branch/Tag",
                hash
            )),
            Some(xs) => {
                self.current = Detached(xs.clone());
                Ok(self.current.clone())
            }
        }
    }

    // Reset

    pub fn reset_soft(&mut self, _hash: NodeHash) -> Result<(Ref, Vec<RepoItem>), String> {
        unimplemented!()
    }

    pub fn reset_hard(&mut self, _hash: NodeHash) -> Ref {
        unimplemented!()
    }

    // Rebasing

    pub fn rebase(&mut self, _hash: NodeHash, _onto: NodeHash) -> Ref {
        unimplemented!()
    }

    // Helpers

    fn find_tag(&self, name: &str) -> Option<Ref> {
        self.refs
            .iter()
            .find(|r| match r {
                Tag(_, tag) => tag == name,
                _ => false,
            })
            .map(|x| x.clone())
    }

    fn find_branch(&self, name: &str) -> Option<Ref> {
        self.refs
            .iter()
            .find(|r| match r {
                Branch(_, br) => br == name,
                _ => false,
            })
            .map(|x| x.clone())
    }

    fn find_commit(&self, hash: NodeHash) -> Option<&Commit<RepoItem>> {
        self.commits.get(&hash)
    }

    /// Find the hashes, from the given hash, on teh specified Reference
    /// This will discard hash nodes until the given hash is found
    /// (ie go back in the history)
    fn find_hashes_from(&self, r: &Ref, hash: NodeHash) -> Option<Hashs> {
        let hashs = r.hashs().skip_while(|x| x != &hash);
        if hashs.is_empty() {
            None
        } else {
            Some(hashs.clone())
        }
    }
}
