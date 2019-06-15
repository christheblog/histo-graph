use crate::core::graph::*;
use crate::core::hashlist::*;
use crate::core::history::Ref::*;
use std::collections::HashMap;
use std::rc::Rc;

type Hashs = Rc<HashList>;

pub struct Repository {
    current: Ref,
    refs: Vec<Ref>,
    commits: HashMap<NodeHash, Commit>,
}

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
pub struct Commit {
    author: Author,
    comment: Comment,
    hash: NodeHash,
    commands: Vec<GraphCommand>,
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

impl Repository {
    /// Creates a new Repository
    pub fn new() -> Repository {
        let master = Branch(HashList::empty(), "master".to_string());
        Repository {
            current: master.clone(),
            refs: vec![master],
            commits: HashMap::new(),
        }
    }

    /// Creates and add a new commit to the current branch
    pub fn commit(
        &mut self,
        command: GraphCommand,
        author: Author,
        comment: Comment,
    ) -> Result<Ref, String> {
        Repository::commit_multiple(self, vec![command], author, comment)
    }

    /// Creates and add a new commit to the current branch
    pub fn commit_multiple(
        &mut self,
        commands: Vec<GraphCommand>,
        author: Author,
        comment: Comment,
    ) -> Result<Ref, String> {
        if self.current.is_read_only() {
            Err(format!("Cannot modify Reference {}", self.current.name()))
        } else {
            let commit = self.create_commit(commands, author, comment);
            let new_head = HashList::cons(commit.hash, self.current.hashs());
            // Updating repo
            self.commits.insert(commit.hash, commit);
            self.current = Branch(new_head.clone(), self.current.name().to_string());
            Ok(self.current.clone())
        }
    }

    fn create_commit(
        &self,
        commands: Vec<GraphCommand>,
        author: Author,
        comment: Comment,
    ) -> Commit {
        let last_hash = self.current.hashs().head_option();
        let commit_hash = Repository::compute_hash(&commands, last_hash);
        Commit {
            author: author,
            comment: comment,
            hash: commit_hash,
            commands: commands,
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

    pub fn checkout_tag(&mut self, name: String) -> Ref {
        unimplemented!()
    }

    pub fn checkout_branch(&mut self, name: String) -> Ref {
        unimplemented!()
    }

    pub fn checkout_hash(&mut self, hash: NodeHash) -> Ref {
        unimplemented!()
    }

    // Reset

    pub fn reset_soft(&mut self, hash: NodeHash) -> (Ref, Vec<GraphCommand>) {
        unimplemented!()
    }

    pub fn reset_hard(&mut self, hash: NodeHash) -> Ref {
        unimplemented!()
    }

    // Rebasing

    pub fn rebase(&mut self, hash: NodeHash, onto: NodeHash) -> Ref {
        unimplemented!()
    }

    /// Compute a hash for the given command, inclusing the previous hash
    fn compute_hash(_command: &Vec<GraphCommand>, _last_hash: Option<NodeHash>) -> NodeHash {
        unimplemented!()
    }
}
