// Declare the submodules (this tells Rust to compile types.rs and state.rs)
pub mod state;
pub mod types;
pub mod masks;
pub mod movegen;
pub mod attacks;
pub mod legality;
pub mod make_move;
pub mod position;
pub mod evaluate;
#[cfg(test)]
mod tests;

// Re-export the main items to make imports much cleaner in main.rs
pub use state::Board;
pub use types::Color;