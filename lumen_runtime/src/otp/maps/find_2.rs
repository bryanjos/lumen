// wasm32 proptest cannot be compiled at the same time as non-wasm32 proptest, so disable tests that
// use proptest completely for wasm32
//
// See https://github.com/rust-lang/cargo/issues/4866
#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use std::convert::TryInto;

use anyhow::*;

use liblumen_alloc::atom;
use liblumen_alloc::erts::exception::{self, *};
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

use lumen_runtime_macros::native_implemented_function;

#[native_implemented_function(find/2)]
pub fn native(process: &Process, key: Term, map: Term) -> exception::Result<Term> {
    let map: Boxed<Map> = map
        .try_into()
        .with_context(|| format!("map ({}) is not a map", map))
        .map_err(|source| badmap(process, map, source.into()))?;

    let result = match map.get(key) {
        Some(term) => {
            let ok = atom!("ok");

            process.tuple_from_slice(&[ok, term])?
        }
        None => atom!("error"),
    };

    Ok(result.into())
}
