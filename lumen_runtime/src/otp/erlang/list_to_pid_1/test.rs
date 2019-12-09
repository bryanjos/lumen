use std::sync::Arc;

use proptest::test_runner::{Config, TestRunner};

use liblumen_alloc::erts::term::prelude::*;
use liblumen_alloc::erts::Node;

use crate::distribution::nodes;
use crate::otp::erlang::list_to_pid_1::native;
use crate::scheduler::{with_process, with_process_arc};
use crate::test::strategy;

#[test]
fn without_list_errors_badarg() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(&strategy::term::is_not_list(arc_process.clone()), |list| {
                prop_assert_badarg!(
                    native(&arc_process, list),
                    format!("string ({}) must be a non-empty list", list)
                );

                Ok(())
            })
            .unwrap();
    });
}

#[test]
fn with_list_encoding_local_pid() {
    with_process(|process| {
        let string = process.charlist_from_str("<").unwrap();
        assert_badarg!(
            native(&process, string),
            format!("string ({}) is missing 'node.number.serial>'", string)
        );

        let string = process.charlist_from_str("<0").unwrap();
        assert_badarg!(
            native(&process, string),
            format!("string ({}) is missing '.number.serial>'", string)
        );

        let string = process.charlist_from_str("<0.").unwrap();
        assert_badarg!(
            native(&process, string),
            format!("string ({}) is missing 'number.serial>'", string)
        );

        let string = process.charlist_from_str("<0.1").unwrap();
        assert_badarg!(
            native(&process, string),
            format!("string ({}) is missing '.serial>", string)
        );

        let string = process.charlist_from_str("<0.1.").unwrap();
        assert_badarg!(
            native(&process, string),
            format!("string ({}) is missing 'serial>", string)
        );

        let string = process.charlist_from_str("<0.1.2").unwrap();
        assert_badarg!(
            native(&process, string),
            format!("string ({}) is missing '>'", string)
        );

        assert_eq!(
            native(&process, process.charlist_from_str("<0.1.2>").unwrap()),
            Ok(Pid::make_term(1, 2).unwrap())
        );

        assert_badarg!(
            native(&process, process.charlist_from_str("<0.1.2>?").unwrap()),
            "extra characters ([63]) beyond end of formatted pid"
        );
    })
}

#[test]
fn with_list_encoding_external_pid_without_known_node_errors_badarg() {
    with_process(|process| {
        let string = process.charlist_from_str("<").unwrap();
        assert_badarg!(
            native(&process, string),
            format!("string ({}) is missing 'node.number.serial>'", string)
        );

        let string = process.charlist_from_str("<2").unwrap();
        assert_badarg!(
            native(&process, string),
            format!("string ({}) is missing '.number.serial>'", string)
        );

        let string = process.charlist_from_str("<2.").unwrap();
        assert_badarg!(
            native(&process, string),
            format!("string ({}) is missing 'number.serial>'", string)
        );

        let string = process.charlist_from_str("<2.3").unwrap();
        assert_badarg!(
            native(&process, string),
            format!("string ({}) is missing '.serial>", string)
        );

        let string = process.charlist_from_str("<2.3.").unwrap();
        assert_badarg!(
            native(&process, string),
            format!("string ({}) is missing 'serial>", string)
        );

        let string = process.charlist_from_str("<2.3.").unwrap();
        assert_badarg!(
            native(&process, string),
            format!("string ({}) is missing 'serial>", string)
        );

        // MUST be a different `id` than other tests that insert the node.
        let arc_node = Arc::new(Node::new(2, Atom::try_from_str("2@external").unwrap(), 0));

        assert_badarg!(
            native(&process, process.charlist_from_str("<2.3.4>").unwrap()),
            "No node with id (2)"
        );

        nodes::insert(arc_node.clone());

        assert_eq!(
            native(&process, process.charlist_from_str("<2.3.4>").unwrap()),
            Ok(process.external_pid(arc_node, 3, 4).unwrap())
        );

        assert_badarg!(
            native(&process, process.charlist_from_str("<2.3.4>?").unwrap()),
            "extra characters ([63]) beyond end of formatted pid"
        );
    });
}
