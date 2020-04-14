#[cfg(test)]
mod test;

use crate::time::datetime;
use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

use lumen_runtime_macros::native_implemented_function;

#[native_implemented_function(universaltime_to_localtime/1)]
pub fn native(process: &Process, unversaltime_tuple: Term) -> exception::Result<Term> {
    let initial_inner_tuple = term_try_into_tuple!(localtime_tuple)?;

    let now: [usize; 6] = datetime::local_now();

    let date_tuple = process.tuple_from_slice(&[
        process.integer(now[0])?,
        process.integer(now[1])?,
        process.integer(now[2])?,
    ])?;
    let time_tuple = process.tuple_from_slice(&[
        process.integer(now[3])?,
        process.integer(now[4])?,
        process.integer(now[5])?,
    ])?;

    process
        .tuple_from_slice(&[date_tuple, time_tuple])
        .map_err(|error| error.into())
}
