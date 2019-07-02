use histo_graph_core::graph::graph::VertexId;

use ring::digest::{Context, SHA256};
use data_encoding::HEXUPPER;

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
    let name = HEXUPPER.encode(digest.as_ref());

    File {
        content,
        name,
    }
}

#[cfg(test)]
mod test {
    use histo_graph_core::graph::graph::VertexId;
    use super::File;
    use super::vertex_to_file;

    use ring::digest::{Context, SHA256};
    use data_encoding::HEXUPPER;

    #[test]
    fn test_hash() {
        let File{content, name} = vertex_to_file(&VertexId(27));

        assert_eq!(name, "4D159113222BFEB85FBE717CC2393EE8A6A85B7CE5AC1791C4EADE5E3DD6DE41")
    }
}
