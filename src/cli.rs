use clap::{CommandFactory, Parser, Subcommand};




use crate::server;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(short, long)]
        debug: bool,
    },
}

pub fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run { debug }) => {
            if *debug {
                println!("RUNNING IN DEBUG MODE");
            };
            server::init();
        }
        None => {
            println!("No Command was inputted: see help below 👇");
            let _ = Cli::command().print_long_help();
        }
    }
}
