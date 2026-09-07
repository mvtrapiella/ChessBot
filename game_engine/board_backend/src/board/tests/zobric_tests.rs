use crate::board::position::{Position, TT_SIZE};
use crate::board::types::Move;
use crate::board::zobric::{Bound, TTEntry};
use super::test_utils::empty_board;

fn dummy_entry(key: u64, depth: u32) -> TTEntry {
    TTEntry {
        key,
        depth,
        bound: Bound::Exact,
        score: 0,
        best_move: Move { origin: 0, destination: 0, promotion: None },
    }
}

#[test]
fn tt_store_keeps_deeper_entry_on_same_key() {
    let mut pos = Position::new(empty_board());
    let hash = 42u64;

    pos.tt_store(hash, dummy_entry(hash, 5));
    pos.tt_store(hash, dummy_entry(hash, 2));

    assert_eq!(pos.tt_probe(hash).unwrap().depth, 5);
}

#[test]
fn tt_store_replaces_same_key_entry_when_at_least_as_deep() {
    let mut pos = Position::new(empty_board());
    let hash = 42u64;

    pos.tt_store(hash, dummy_entry(hash, 5));
    pos.tt_store(hash, dummy_entry(hash, 5));

    assert_eq!(pos.tt_probe(hash).unwrap().depth, 5);
}

#[test]
fn tt_store_overwrites_on_index_collision_regardless_of_depth() {
    let mut pos = Position::new(empty_board());
    let hash_a = 42u64;
    // Same slot as hash_a (differs by exactly TT_SIZE, a power of two), but represents
    // a different position -- hash & (TT_SIZE - 1) is identical for both.
    let hash_b = hash_a + TT_SIZE as u64;

    pos.tt_store(hash_a, dummy_entry(hash_a, 20));
    pos.tt_store(hash_b, dummy_entry(hash_b, 1));

    assert_eq!(pos.tt_probe(hash_b).unwrap().depth, 1);
}

#[test]
fn tt_probe_misses_when_slot_holds_a_different_position() {
    let mut pos = Position::new(empty_board());
    let hash_a = 42u64;
    let hash_b = hash_a + TT_SIZE as u64;

    pos.tt_store(hash_a, dummy_entry(hash_a, 10));

    // hash_b was never stored, but collides into hash_a's slot -- must read as a miss,
    // not as a false hit for hash_a's entry.
    assert!(pos.tt_probe(hash_b).is_none());
}
