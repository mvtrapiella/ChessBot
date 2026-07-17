// Declare the submodules (this tells Rust to compile types.rs and state.rs)
pub mod state;
pub mod types;
pub mod marks;

// Re-export the main items to make imports much cleaner in main.rs
pub use state::Board;
pub use types::Color;