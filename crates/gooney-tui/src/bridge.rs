// crates/gooney-tui/src/bridge.rs
use std::os::raw::c_int;

extern "C" {
    pub fn xzxt_sim_init();
    pub fn xzxt_sim_destroy();
    pub fn xzxt_sim_step(branch_taken: c_int, target_pc: u32, stall: c_int, flush: c_int);
    
    pub fn xzxt_get_pc() -> u32;
    pub fn xzxt_get_wb_result() -> u64;
    pub fn xzxt_get_wb_rd() -> u8;
    pub fn xzxt_get_wb_commit() -> c_int;
    #[allow(dead_code)]
    pub fn xzxt_get_eflags() -> u64;

    pub fn xzxt_get_mem_ren() -> c_int;
    pub fn xzxt_get_mem_wen() -> c_int;
    pub fn xzxt_get_mem_addr() -> u32;
    pub fn xzxt_get_mem_wdata() -> u64;
    pub fn xzxt_get_mem_rdata() -> u64;
}
