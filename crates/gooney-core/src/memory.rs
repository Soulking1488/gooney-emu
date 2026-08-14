pub struct Memory {
    data: Vec<u8>,
    base_addr: u64,
}

impl Memory {
    /// Initialize memory with a given size (e.g., 16MB) starting at 0x80000000
    pub fn new(size: usize, base_addr: u64) -> Self {
        Self {
            data: vec![0; size],
            base_addr,
        }
    }

    /// Translate virtual/physical address to internal vector index
    fn translate(&self, addr: u64) -> Result<usize, &'static str> {
        if addr < self.base_addr || addr >= self.base_addr + self.data.len() as u64 {
            Err("Memory Access Out of Bounds")
        } else {
            Ok((addr - self.base_addr) as usize)
        }
    }

    pub fn load_binary(&mut self, offset: u64, bin: &[u8]) -> Result<(), &'static str> {
        let idx = self.translate(offset)?;
        if idx + bin.len() > self.data.len() {
            return Err("Binary exceeds memory capacity");
        }
        self.data[idx..idx + bin.len()].copy_from_slice(bin);
        Ok(())
    }

    pub fn read_u32(&self, addr: u64) -> Result<u32, &'static str> {
        let idx = self.translate(addr)?;
        if idx + 4 > self.data.len() {
            return Err("Unaligned or out-of-bounds 32-bit read");
        }
        let bytes = &self.data[idx..idx + 4];
        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn write_u32(&mut self, addr: u64, val: u32) -> Result<(), &'static str> {
        let idx = self.translate(addr)?;
        if idx + 4 > self.data.len() {
            return Err("Unaligned or out-of-bounds 32-bit write");
        }
        let bytes = val.to_le_bytes();
        self.data[idx..idx + 4].copy_from_slice(&bytes);
        Ok(())
    }
}
