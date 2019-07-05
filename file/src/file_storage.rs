use histo_graph_core::graph::graph::VertexId;

use ring::digest::{Context, SHA256};
use data_encoding::HEXLOWER;

use std::path::Path;
use futures::future::Future;

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

fn write_file_in_dir(dir_path: &Path, file: File) -> impl Future {
    let path = dir_path.join(&file.name);
    tokio_fs::write(path, file.content)
}



fn write_all_vertices_to_files<I>(i: I) -> impl Future
    where I: Iterator,
          <I as Iterator>::Item: AsRef<VertexId>
{
    let futs = i
        .map(| v | vertex_to_file(v.as_ref()))
        .map(| f | write_file_in_dir(Path::new("some_path") , f)) ;

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
    use super::File;
    use super::vertex_to_file;
    use super::write_file;
    use futures::future::Future;

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
}
