use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "varve", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>
}

#[derive(Subcommand)]
pub enum Command {
    Deposit,
    Excavate,

    Construct {
        #[command(subcommand)]
        command: ConstructCommand
    },

    Dossier,
    Profile,

    Acquire,
    Publish
}

#[derive(Subcommand)]
pub enum ConstructCommand {
    Action,
    Event,
    Dossier
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Command::Deposit => {}
            Command::Excavate => {}

            Command::Construct { command } => match command {
                ConstructCommand::Action => {}
                ConstructCommand::Event => {}
                ConstructCommand::Dossier => {}
            }

            Command::Dossier => {}
            Command::Profile => {}

            Command::Acquire => {}
            Command::Publish => {}
        }
    }
    else {
        Cli::command().print_help().unwrap();
    }
}
