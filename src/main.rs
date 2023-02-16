use clap::{command, Parser, Subcommand};
use cmd::list::print_list;

mod cmd {
    pub mod list;
}
mod util;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List installed formulae and casks.
    List {
        /// Force output to be one entry per line.
        #[arg(short = '1', long)]
        oneline: bool,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::List { oneline } => {
            print_list(oneline);
        }
    };
}
