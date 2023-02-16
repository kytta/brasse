use brasse::casks::get_casks;
use brasse::formulae::get_formulae;
use brasse::util::output::print_list;
use clap::{command, Parser, Subcommand};

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
            print_list(get_formulae().unwrap(), oneline, Some("Formulae"));
            print_list(get_casks().unwrap(), oneline, Some("Casks"));
        }
    };
}
