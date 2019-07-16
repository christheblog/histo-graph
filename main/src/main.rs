use clap::{App, SubCommand};

fn main() {
    let matches = App::new("histo-graph")
        .version("0.1.0")
        .about("Historizes graphs")
        .subcommand(SubCommand::with_name("show")
            .about("shows a graph")
            )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("show") {
        println!("Show the graph");
    }
}
