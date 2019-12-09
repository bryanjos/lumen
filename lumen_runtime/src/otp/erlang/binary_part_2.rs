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
use liblumen_alloc::erts::term::prelude::*;

use lumen_runtime_macros::native_implemented_function;

use crate::otp::erlang;

#[native_implemented_function(binary_part/2)]
pub fn native(process: &Process, binary: Term, start_length: Term) -> exception::Result<Term> {
    let start_length_tuple: Boxed<Tuple> = start_length
        .try_into()
        .with_context(|| format!("start_length ({}) is not a tuple", start_length))?;

    if start_length_tuple.len() == 2 {
        erlang::binary_part_3::native(
            process,
            binary,
            start_length_tuple[0],
            start_length_tuple[1],
        )
    } else {
        Err(anyhow!(
            "start_length ({}) is a tuple, but not 2-arity",
            start_length
        )
        .into())
    }
}
