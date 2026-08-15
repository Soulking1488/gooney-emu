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

    // --- 8-bit ---
    pub fn read_u8(&self, addr: u64) -> Result<u8, &'static str> {
        let idx = self.translate(addr)?;
        Ok(self.data[idx])
    }

    pub fn write_u8(&mut self, addr: u64, val: u8) -> Result<(), &'static str> {
        let idx = self.translate(addr)?;
        self.data[idx] = val;
        Ok(())
    }

    // --- 16-bit ---
    pub fn read_u16(&self, addr: u64) -> Result<u16, &'static str> {
        let idx = self.translate(addr)?;
        if idx + 2 > self.data.len() {
            return Err("Out-of-bounds 16-bit read");
        }
        let bytes = &self.data[idx..idx + 2];
        Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn write_u16(&mut self, addr: u64, val: u16) -> Result<(), &'static str> {
        let idx = self.translate(addr)?;
        if idx + 2 > self.data.len() {
            return Err("Out-of-bounds 16-bit write");
        }
        let bytes = val.to_le_bytes();
        self.data[idx..idx + 2].copy_from_slice(&bytes);
        Ok(())
    }

    // --- 32-bit ---
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

    // --- 64-bit ---
    pub fn read_u64(&self, addr: u64) -> Result<u64, &'static str> {
        let idx = self.translate(addr)?;
        if idx + 8 > self.data.len() {
            return Err("Unaligned or out-of-bounds 64-bit read");
        }
        let bytes = &self.data[idx..idx + 8];
        Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn write_u64(&mut self, addr: u64, val: u64) -> Result<(), &'static str> {
        let idx = self.translate(addr)?;
        if idx + 8 > self.data.len() {
            return Err("Unaligned or out-of-bounds 64-bit write");
        }
        let bytes = val.to_le_bytes();
        self.data[idx..idx + 8].copy_from_slice(&bytes);
        Ok(())
    }
}
