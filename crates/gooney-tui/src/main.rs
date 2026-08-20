mod cli;
mod handler;
mod commands;
mod bridge;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    handler::handle_command(&cli.command);
}
