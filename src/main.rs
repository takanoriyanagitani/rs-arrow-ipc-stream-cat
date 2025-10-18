use clap::Parser;
use rs_arrow_ipc_stream_cat::{show_records, stdin2records};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    fields: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let projection = args
        .fields
        .map(|s| -> Result<Vec<usize>, std::io::Error> {
            s.split(',')
                .map(|s| {
                    s.parse::<usize>()
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))
                })
                .collect()
        })
        .transpose()?;

    let rbats = stdin2records(projection)?;
    show_records(rbats)
}
