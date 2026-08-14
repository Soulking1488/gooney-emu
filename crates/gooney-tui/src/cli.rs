use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gooney")]
#[command(author = "Silicon IP Developer")]
#[command(version = "0.1.0")]
#[command(about = "Gooney-emu: Minimalist RV64I Simulator & Silicon IP Validator", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Interactive REPL shell for running RISC-V instructions
    Shell,

    /// Run a script, raw binary, or assembly file
    Run {
        #[arg(help = "Path to the binary or script file")]
        path: String,
    },

    /// Pre-flight check and compile a custom extension folder (e.g., custom-0)
    Lint {
        #[arg(help = "Target extension name (e.g., custom-0, custom-1)")]
        extension: String,
    },

    /// Automated trace diffing against Verilator RTL simulation logs
    Diff {
        #[arg(help = "Path to emulator retirement trace log")]
        emu_trace: String,
        #[arg(help = "Path to RTL simulation trace log")]
        rtl_trace: String,
    },

    /// Run official RISC-V architectural compliance test suite
    Test,

    /// Disassemble raw binary or ELF with custom opcode awareness
    Disasm {
        #[arg(help = "Path to target binary file")]
        path: String,
    },

    /// Stress-test hazard detection and pipeline forwarding logic via fuzzing
    Fuzz {
        #[arg(short, long, default_value_t = 1000, help = "Number of random instruction cycles")]
        count: usize,
    },
}
