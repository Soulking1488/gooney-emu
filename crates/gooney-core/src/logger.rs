use std::fs::File;
use std::io::{self, Write};

pub struct TraceLogger {
    file: Option<File>,
    enabled: bool,
}

impl TraceLogger {
    pub fn new() -> Self {
        Self {
            file: None,
            enabled: false,
        }
    }

    /// Enable file logging for trace comparisons (e.g., for `gooney-diff`)
    pub fn enable_file_logging(&mut self, path: &str) -> io::Result<()> {
        let file = File::create(path)?;
        self.file = Some(file);
        self.enabled = true;
        Ok(())
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.file = None;
    }

    /// Log a successfully retired instruction
    pub fn log_retirement(&mut self, pc: u64, instruction: u32, rd: usize, result_val: u64) {
        if !self.enabled {
            return;
        }

        // Standardized trace format: [PC] HEX_INST -> xRD = VALUE
        let log_line = format!(
            "[PC: 0x{:016X}] Inst: 0x{:08X} | x{:02} <- 0x{:016X}\n",
            pc, instruction, rd, result_val
        );

        // Print to stdout or write to file if configured
        if let Some(ref mut f) = self.file {
            let _ = f.write_all(log_line.as_bytes());
        } else {
            print!("{}", log_line);
        }
    }
}
