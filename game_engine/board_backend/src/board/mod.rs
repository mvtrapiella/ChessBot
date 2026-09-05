// Declare the topic-grouped submodules
pub mod model;
pub mod movegen;
pub mod game;
pub mod search;
#[cfg(test)]
mod tests;

// Re-export the leaf modules so existing `crate::board::X` paths (state, types,
// position, make_move, evaluate, negamax, zobric) keep resolving the same
// regardless of which topic folder they physically live in.
pub use model::state;
pub use model::types;
pub use game::position;
pub use game::make_move;
pub use search::evaluate;
pub use search::negamax;
pub use search::opening_book;
pub use search::polyglot_hash;
pub use search::zobric;

// Re-export the main items to make imports much cleaner in main.rs
pub use state::Board;
pub use types::Color;