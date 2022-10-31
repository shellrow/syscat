pub mod cpu;
pub mod disk;
pub mod memory;
pub mod network;
pub mod process;
pub mod user;
pub mod system;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
