use std::ops::RangeInclusive;
use std::sync::Arc;

use proptest::collection::SizeRange;
use proptest::strategy::{BoxedStrategy, Just, Strategy};

use crate::process::{self, Process};
use crate::term::Term;

pub mod byte_vec;
pub mod size_range;
pub mod term;

pub const NON_EMPTY_RANGE_INCLUSIVE: RangeInclusive<usize> = 1..=MAX_LEN;

pub fn bits_to_bytes(bits: usize) -> usize {
    (bits + 7) / 8
}

pub fn byte_vec() -> BoxedStrategy<Vec<u8>> {
    byte_vec::with_size_range(RANGE_INCLUSIVE.into())
}

pub fn process() -> BoxedStrategy<Arc<Process>> {
    Just(process::local::test_init())
        .prop_flat_map(|init_arc_process| {
            // generated in a prop_flat_map instead of prop_map so that a new process is generated
            // for each test so that a prior run's register doesn't interfere
            Just(process::local::test(&init_arc_process))
        })
        .boxed()
}

pub fn term(arc_process: Arc<Process>) -> BoxedStrategy<Term> {
    let container_arc_process = arc_process.clone();

    term::leaf(RANGE_INCLUSIVE, arc_process)
        .prop_recursive(
            DEPTH,
            (MAX_LEN * (DEPTH as usize + 1)) as u32,
            MAX_LEN as u32,
            move |element| {
                term::container(
                    element,
                    RANGE_INCLUSIVE.clone().into(),
                    container_arc_process.clone(),
                )
            },
        )
        .boxed()
}

const DEPTH: u32 = 3;
const MAX_LEN: usize = 3;
const RANGE_INCLUSIVE: RangeInclusive<usize> = 0..=MAX_LEN;

pub fn size_range() -> SizeRange {
    RANGE_INCLUSIVE.clone().into()
}
