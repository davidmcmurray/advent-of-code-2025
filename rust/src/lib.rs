pub use utils::*;
pub mod utils {
    pub fn load_puzzle_bytes(day: u8) -> Vec<u8> {
        std::fs::read(&format!("../inputs/{day}.txt")).unwrap()
    }
    pub fn load_puzzle_string(day: u8) -> String {
        std::fs::read_to_string(&format!("../inputs/{day}.txt")).unwrap()
    }
}
