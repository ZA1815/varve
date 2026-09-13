use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "varve", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>
}

#[derive(Subcommand)]
pub enum Command {
    // addressing -> stream
    Deposit,
    Stream,
    Designate,

    // varve
    Dossier,
    Profile,

    // basin
    Acquire,
    Publish
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        
    }
    else {
        Cli::command().print_help().unwrap();
    }
}
