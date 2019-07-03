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
    let f = f.and_then(|dir| {
        println!("created directory");
        Ok(())
    });
    let f = f.map_err(|e| println!("Error"));

    tokio::run(f);
}

#[cfg(test)]
mod test {
    use histo_graph_core::graph::graph::VertexId;
    use super::File;
    use super::vertex_to_file;
    use super::create_dir;

    #[test]
    fn test_hash() {
        let File{content, name} = vertex_to_file(&VertexId(27));

        assert_eq!(name, "4d159113222bfeb85fbe717cc2393ee8a6a85b7ce5ac1791c4eade5e3dd6de41")
    }

    #[test]
    fn test_create_dir() {
        create_dir();
    }
}
