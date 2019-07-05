use histo_graph_core::graph::graph::VertexId;

use ring::digest::{Context, SHA256};
use data_encoding::HEXLOWER;

use std::path::{Path, PathBuf};
use futures::future::Future;
use std::{
    borrow::Borrow,
    io
};

struct File {
    content: Vec<u8>,
    name: String,
}

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

fn vertex_to_file(vertex_id: &VertexId) -> File {
    // serialize the vertex_id
    let content: Vec<u8> = bincode::serialize(&vertex_id.0).unwrap();
    let hash: Hash = (&content).into();

    File {
        content,
        name: hash.to_string(),
    }
}

fn write_file_in_dir(dir_path: &Path, file: File) -> impl Future<Error = io::Error> {
    let path = dir_path.join(&file.name);
    tokio_fs::write(path, file.content)
}


fn write_all_vertices_to_files<I>(path: PathBuf, i: I) -> impl Future<Item=Vec<()>, Error = io::Error>
    where I: Iterator,
          <I as Iterator>::Item: Borrow<VertexId>
{
    let futs = i
        .map(| v | vertex_to_file(v.borrow()))
        .map(move | f | write_file_in_dir(path.as_ref(), f).and_then(| _ | Ok(()))) ;

    futures::future::join_all(futs)
}

fn write_file(file: File) -> impl Future {
    let path = Path::new("./foo");

    tokio_fs::create_dir_all(path)
        .and_then(move | _ | {
            let path = path.join(&file.name);
            tokio_fs::write(path, file.content)
        })
}

#[cfg(test)]
mod test {
    use histo_graph_core::graph::graph::VertexId;
    use super::{File,
                vertex_to_file,
                write_file,
                write_all_vertices_to_files};
    use futures::future::Future;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_hash() {
        let File{content: _, name} = vertex_to_file(&VertexId(27));

        assert_eq!(name, "4d159113222bfeb85fbe717cc2393ee8a6a85b7ce5ac1791c4eade5e3dd6de41")
    }

    #[test]
    fn test_write_vertex() {
        let file = vertex_to_file(&VertexId(27));

        let f = write_file(file)
            .map(| _ | ())
            .map_err(| _ | eprintln!("Error"));

        tokio::run(f);
    }

    /*fn the_future() -> impl Future<Item = (), Error = ()> {
        let vertices = vec!{VertexId(1), VertexId(2), VertexId(3), VertexId(4)};
        let iter = vertices.iter();

        let path: PathBuf = Path::new("foo/").into();

        let f = write_all_vertices_to_files(path, iter);

        let f = f
            .and_then(| _ | { println!("Done"); Ok(()) })
            .map_err(| _ | eprintln!("Error"));

        f
    }*/

    #[test]
    fn test_write_vertices() {
        let vertices = vec!{VertexId(1), VertexId(2), VertexId(3), VertexId(4)};

        let path: PathBuf = Path::new("foo/").into();

        let f = write_all_vertices_to_files(path, vertices.into_iter());

        let f = f
            .and_then(| _ | { println!("Done"); Ok(()) })
            .map_err(| _ | eprintln!("Error"));


        tokio::run(f);
    }
}
