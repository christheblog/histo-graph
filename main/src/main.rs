use clap::{App, SubCommand};
use histo_graph_file::file_storage::load_graph;
use std::path::{PathBuf, Path};
use histo_graph_serde::directed_graph_serde::DirectedGraphSer;
use std::ffi::OsString;
use tokio::runtime::Runtime;

fn main() {
    let matches = App::new("histo-graph")
        .version("0.1.0")
        .about("Historizes graphs")
        .subcommand(SubCommand::with_name("show")
            .about("shows a graph")
            )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("show") {
        let base_dir: PathBuf = Path::new("../target/test/store/").into();
        let f = load_graph(base_dir, &OsString::from("laurengraph"));

        let mut rt = Runtime::new().unwrap();
        let graph = rt.block_on(f).unwrap();
        let ser: DirectedGraphSer = (&graph).into();
        let str = serde_json::to_string(&ser).unwrap();
        println!("{}", str);
    }
}
