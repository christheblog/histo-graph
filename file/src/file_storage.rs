use histo_graph_core::graph::{
    graph::VertexId,
    directed_graph::DirectedGraph,
};

use crate::error::{Error, Result};

use ring::digest::{Context, SHA256};
use data_encoding::HEXLOWER;
use serde::{Serialize, Deserialize};

use futures::future::Future;
use std::{
    borrow::Borrow,
    io,
    path::{Path, PathBuf},
};


#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Hash([u8; 32]);

impl Hash {
    fn to_string(&self) -> String {
        HEXLOWER.encode(&self.0)
    }
}

impl<T> From<T> for Hash
    where T: AsRef<[u8]> {
    fn from(content: T) -> Hash {
        let mut context = Context::new(&SHA256);
        context.update(content.as_ref());
        let digest = context.finish();
        let mut hash: [u8; 32] = [0u8; 32];
        hash.copy_from_slice(digest.as_ref());

        Hash(hash)
    }
}

struct File {
    content: Vec<u8>,
    hash: Hash,
}

fn vertex_to_file(vertex_id: &VertexId) -> File {
    // serialize the vertex_id
    let content: Vec<u8> = bincode::serialize(&vertex_id.0).unwrap();
    let hash: Hash = (&content).into();

    File {
        content,
        hash,
    }
}

fn hash_vec_to_file(hash_vec: &Vec<Hash>) -> File {
    // serialize the vertex_id
    let content: Vec<u8> = bincode::serialize(&hash_vec).unwrap();
    let hash: Hash = (&content).into();

    File {
        content,
        hash,
    }
}

fn file_to_vertex(file: &File) -> Result<VertexId> {
    let id: u64 = bincode::deserialize(file.content.as_ref())?;
    Ok(VertexId(id))
}

fn file_to_hash_vec(file: &File) -> Result<Vec<Hash>> {
    let result = bincode::deserialize(file.content.as_ref())?;
    Ok(result)
}

fn write_file_in_dir(dir_path: &Path, file: File) -> impl Future<Error = io::Error> {
    let path = dir_path.join(&file.hash.to_string());
    tokio_fs::write(path, file.content)
}

/// Writes vertices to files.
///
/// First creates a sub-directory `vertex/` in the provided `base_path`, then writes the vertices
/// into this sub-directory, creating one file for each vertex.
/// Returns a vector of the hashes of the written files.
fn write_all_vertices_to_files<I>(base_path: PathBuf, i: I) -> impl Future<Item=Vec<Hash>, Error = io::Error>
    where I: IntoIterator,
          <I as IntoIterator>::Item: Borrow<VertexId>
{
    let path = base_path.join("vertex");
    let futs = i
        .into_iter()
        .map(| v | vertex_to_file(v.borrow()))
        .map({
            let path = path.clone();
            move |f| {
                let hash = f.hash;
                write_file_in_dir(path.as_ref(), f)
                    .map(move |_| hash)
            }
        });

    tokio_fs::create_dir_all(path)
        .and_then(| _ | futures::future::join_all(futs))
}

/// Writes the vector of hashes of the vertices of a graph to a file.
///
/// First creates a sub-directoy `vertexvec/` in the provided `base_path`, then writes the vector
/// of hashes into a single file in that sub-directory.
/// Returns a hash of the written file.
fn write_vertex_hash_vec_file(base_path: PathBuf, hash_vec: Vec<Hash>) -> impl Future<Item = Hash, Error = io::Error> {
    let path = base_path.join("vertexvec");
    let file = hash_vec_to_file(&hash_vec);
    let hash = file.hash;

    tokio_fs::create_dir_all(path.clone())
        .and_then(move | _ | write_file_in_dir(&path, file))
        .map( move | _ | hash)
}

/// Stores the vertices of a graph. Returns Future of the hash of the vertex vector file.
///
///
fn store_graph_vertices(base_path: PathBuf, graph: &DirectedGraph) -> impl Future<Item = Hash, Error = io::Error> {
    let vertices: Vec<VertexId> = graph
        .vertices()
        .map(| v | *v)
        .collect();

    tokio_fs::create_dir_all(base_path.clone())
        .and_then({ let base_path = base_path.clone(); move | _ | {
            write_all_vertices_to_files(base_path, vertices)
        }})
        .and_then(move | hash_vec |
            write_vertex_hash_vec_file(base_path, hash_vec)
        )
}

fn read_file_in_dir(dir_path: &Path, hash: Hash) -> impl Future<Item = File, Error = io::Error> {
    let path = dir_path.join(hash.to_string());
    tokio_fs::read(path)
        .map( move |content| File {
            content,
            hash
        })
}

/// Reads a vertex hash vector file.
///
/// Reads from a file placed in the sub-directory `vertexvec/` of the provided base_path, with the
/// provided `hash` as a filename.
/// Returns a hash vector.
fn read_vertex_hash_vec(base_path: PathBuf, hash: Hash) -> impl Future<Item = Vec<Hash>, Error = Error> {
    let path = base_path
        .join("vertexvec");

    read_file_in_dir(&path, hash)
        .map_err(Into::into)
        .and_then(|file| file_to_hash_vec(&file) )
}

fn read_all_vertices_from_files(base_path: PathBuf, hash_vec: Vec<Hash>) -> impl Future<Item = Vec<VertexId>, Error = Error> {
    let path = base_path.join("vertex");

    let futs = hash_vec
        .into_iter()
        .map(move |hash| {
            read_file_in_dir(&path, hash)
                .map_err(Into::into)
                .and_then(|file| file_to_vertex(&file))
        });

    futures::future::join_all(futs)
}

#[cfg(test)]
mod test {
    use histo_graph_core::graph::graph::VertexId;
    use super::{File,
                vertex_to_file,
                write_all_vertices_to_files};
    use futures::future::Future;
    use tokio::runtime::Runtime;
    use std::path::{Path, PathBuf};
    use histo_graph_core::graph::directed_graph::DirectedGraph;
    use crate::error::{Error, Result};
    use crate::file_storage::{store_graph_vertices, write_file_in_dir, read_file_in_dir, file_to_vertex, read_vertex_hash_vec, read_all_vertices_from_files};

    #[test]
    fn test_hash() {
        let File{content: _, hash} = vertex_to_file(&VertexId(27));

        assert_eq!(hash.to_string(), "4d159113222bfeb85fbe717cc2393ee8a6a85b7ce5ac1791c4eade5e3dd6de41")
    }

    #[test]
    fn test_write_and_read_vertex() -> Result<()> {
        let vertex = VertexId(18);

        let file = vertex_to_file(&vertex);
        let hash = file.hash;

        let path: PathBuf = Path::new("../target/test/store/").into();

        let f = write_file_in_dir(&path, file)
            .and_then(move | _ | read_file_in_dir(&path, hash));

        let mut rt = Runtime::new()?;
        let file = rt.block_on(f)?;

        let result = file_to_vertex(&file)?;

        assert_eq!(result, vertex);

        Ok(())
    }

    #[test]
    fn test_write_vertices() -> Result<()> {
        let vertices = vec!{VertexId(1), VertexId(2), VertexId(3), VertexId(4)};

        let path: PathBuf = Path::new("../target/test/store/").into();

        let f = write_all_vertices_to_files(path, vertices.into_iter());

        let mut rt = Runtime::new()?;
        rt.block_on(f)?;

        Ok(())
    }

    #[test]
    fn test_store_graph_vertices() -> Result<()> {
        let mut graph = DirectedGraph::new();
        graph.add_vertex(VertexId(27));
        graph.add_vertex(VertexId(28));
        graph.add_vertex(VertexId(29));

        let path: PathBuf = Path::new("../target/test/store/").into();

        let f = store_graph_vertices(path.clone(), &graph)
            .map_err(Into::into)
            .and_then({ let path = path.clone(); move |hash| read_vertex_hash_vec(path, hash)})
            .and_then(move |hash_vec| read_all_vertices_from_files(path, hash_vec));

        let mut rt = Runtime::new()?;
        let vertices = rt.block_on(f)?;

        let mut result_graph = DirectedGraph::new();
        for v in vertices {
            result_graph.add_vertex(v);
        }

        assert_eq!(graph, result_graph);

        Ok(())
    }
}
