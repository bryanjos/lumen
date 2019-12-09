// wasm32 proptest cannot be compiled at the same time as non-wasm32 proptest, so disable tests that
// use proptest completely for wasm32
//
// See https://github.com/rust-lang/cargo/issues/4866
#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use std::convert::TryInto;

use anyhow::*;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::index::OneBasedIndex;
use liblumen_alloc::erts::term::prelude::*;

use lumen_runtime_macros::native_implemented_function;

/// `delete_element/2`
#[native_implemented_function(delete_element/2)]
pub fn native(process: &Process, index: Term, tuple: Term) -> exception::Result<Term> {
    let initial_inner_tuple: Boxed<Tuple> = tuple
        .try_into()
        .with_context(|| format!("tuple ({}) must be a tuple", tuple))?;
    let initial_len = initial_inner_tuple.len();
    let index_zero_based: OneBasedIndex = index
        .try_into()
        .with_context(|| format!("index must be 1-based index between 1-{}", initial_len))?;

    if index_zero_based < initial_len {
        let smaller_len = initial_len - 1;
        let smaller_element_iterator =
            initial_inner_tuple
                .iter()
                .enumerate()
                .filter_map(|(old_index, old_term)| {
                    if index_zero_based == old_index {
                        None
                    } else {
                        Some(*old_term)
                    }
                });
        let smaller_tuple = process.tuple_from_iter(smaller_element_iterator, smaller_len)?;

        Ok(smaller_tuple)
    } else {
        Err(TryIntoIntegerError::OutOfRange)
            .with_context(|| format!("index must be 1-based index between 1-{}", initial_len))
            .map_err(From::from)
    }
}
