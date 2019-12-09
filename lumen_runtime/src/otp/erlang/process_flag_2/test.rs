mod with_atom_flag;

use proptest::prop_assert_eq;
use proptest::strategy::Strategy;
use proptest::test_runner::{Config, TestRunner};

use crate::otp::erlang::process_flag_2::native;
use crate::scheduler::{with_process, with_process_arc};
use crate::test::strategy;

#[test]
fn without_atom_flag_errors_badarg() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::is_not_atom(arc_process.clone()),
                    strategy::term(arc_process.clone()),
                ),
                |(flag, value)| {
                    prop_assert_badarg!(
                        native(&arc_process, flag, value),
                        format!("flag ({}) must be an atom", flag)
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}
