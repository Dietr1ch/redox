use schemes::Result;

pub mod ahci;
pub mod ide;

pub trait Disk {
    fn read(&mut self, block: u64, buffer: &mut [u8]) -> Result<usize>;
    fn write(&mut self, block: u64, buffer: &[u8]) -> Result<usize>;
}
