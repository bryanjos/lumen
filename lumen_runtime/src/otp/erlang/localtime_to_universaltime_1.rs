#[cfg(test)]
mod test;

use crate::time::datetime;
use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

use lumen_runtime_macros::native_implemented_function;

#[native_implemented_function(localtime_to_universaltime/1)]
pub fn native(process: &Process, localtime_tuple: Term) -> exception::Result<Term> {
    let tuple: Result<Boxed<Tuple>> = term.decode().unwrap().try_into()
    let date_tuple = tuple.unwrap().elements()[0].decode().unwrap().try_into().unwrap().elements()
    let time_tuple = tuple.unwrap().elements()[1].decode().unwrap().try_into().unwrap().elements()

    let time_slice = &mut[date_tuple, time_tuple].concat()

    for elem in time_slice.iter_mut() {
        *elem = elem.unwrap().try_into()
    }

    let universal_time: [usize; 6] = datetime::local_datetime_to_utc_datetime(time_slice)

    let date_tuple = process.tuple_from_slice(&[
        process.integer(universal_time[0])?,
        process.integer(universal_time[1])?,
        process.integer(universal_time[2])?,
    ])?;
    let time_tuple = process.tuple_from_slice(&[
        process.integer(universal_time[3])?,
        process.integer(universal_time[4])?,
        process.integer(universal_time[5])?,
    ])?;

    process
        .tuple_from_slice(&[date_tuple, time_tuple])
        .map_err(|error| error.into())
}
