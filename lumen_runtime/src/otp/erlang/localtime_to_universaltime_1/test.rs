use std::convert::TryInto;

use proptest::strategy::Just;
use proptest::{prop_assert, prop_assert_eq};

use liblumen_alloc::erts::term::prelude::*;

use crate::otp::erlang::localtime_0;
use crate::otp::erlang::localtime_to_universaltime_1;
use crate::otp::erlang::universaltime_to_localtime_1;
use crate::test::strategy;

#[test]
fn turns_localtime_to_universal_time_and_back() {
    run!(
        |arc_process| { (Just(arc_process.clone())) },
        |(arc_process)| {
            let localtime = localtime_0::native(arc_process);
            let universaltime = localtime_to_universaltime_1::native(arc_process, localtime)
            let new_localtime = universaltime_to_localtime_1::native(arc_process, universaltime)

            prop_assert_eq!(localtime, new_localtime);
            Ok(())
        },
    );
}
