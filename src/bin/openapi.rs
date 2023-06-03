use std::io::stdout;

use api::ApiDocs;
use clap::Parser;
use utoipa::OpenApi;

#[derive(Debug, Parser)]
struct Cli {
    /// Format the JSON output
    #[arg(short, long)]
    pretty: bool,
}

fn main() -> serde_json::Result<()> {
    let args = Cli::parse();

    let api_docs = ApiDocs::openapi();

    if args.pretty {
        serde_json::to_writer_pretty(stdout(), &api_docs)
    } else {
        serde_json::to_writer(stdout(), &api_docs)
    }
}
