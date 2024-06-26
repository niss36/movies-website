use std::io::stdout;

use clap::Parser;
use movies_api::get_api_docs;

#[derive(Debug, Parser)]
struct Cli {
    /// Format the JSON output
    #[arg(short, long)]
    pretty: bool,
}

fn main() -> serde_json::Result<()> {
    let args = Cli::parse();

    let api_docs = get_api_docs();

    if args.pretty {
        serde_json::to_writer_pretty(stdout(), &api_docs)
    } else {
        serde_json::to_writer(stdout(), &api_docs)
    }
}
