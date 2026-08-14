use crate::cli::Commands;
use crate::commands;

pub fn handle_command(command: &Commands) {
    match command {
        Commands::Shell => commands::shell::execute(),
        Commands::Run { path } => commands::run::execute(path),
        Commands::Lint { extension } => commands::lint::execute(extension),
        Commands::Diff { emu_trace, rtl_trace } => commands::diff::execute(emu_trace, rtl_trace),
        Commands::Test => commands::test_cmd::execute(),
        Commands::Disasm { path } => commands::disasm::execute(path),
        Commands::Fuzz { count } => commands::fuzz::execute(*count),
    }
}
