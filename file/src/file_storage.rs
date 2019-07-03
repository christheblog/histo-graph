use histo_graph_core::graph::graph::VertexId;

use ring::digest::{Context, SHA256};
use data_encoding::HEXLOWER;

use std::path::Path;
use futures::future::Future;
use tokio_fs::CreateDirFuture;

struct File {
    content: Vec<u8>,
    name: String,
}

fn vertex_to_file(vertex_id: &VertexId) -> File {
    // serialize the vertex_id
    let content: Vec<u8> = bincode::serialize(&vertex_id.0).unwrap();

    // create a SHA256 hash of the serialized vertex_id
    let mut context = Context::new(&SHA256);
    context.update(&content);
    let digest = context.finish();
    let name = HEXLOWER.encode(digest.as_ref());

    File {
        content,
        name,
    }
}

fn create_dir() {
    let path = Path::new("./foo");
    let f = tokio_fs::create_dir(path);
    let f = f.and_then(| _ | {
        println!("created directory");
        Ok(())
    });
    let f = f.map_err(|_| eprintln!("Error"));

    tokio::run(f);
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
    use super::create_dir;
    use super::write_file;
    use futures::future::Future;

    #[test]
    fn test_hash() {
        let File{content, name} = vertex_to_file(&VertexId(27));

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
