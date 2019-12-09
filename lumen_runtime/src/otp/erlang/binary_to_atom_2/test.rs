use std::convert::TryInto;

use proptest::prop_assert_eq;
use proptest::test_runner::{Config, TestRunner};

use liblumen_alloc::erts::term::prelude::*;

use crate::otp::erlang::binary_to_atom_2::native;
use crate::scheduler::with_process_arc;
use crate::test::strategy;

#[test]
fn without_binary_errors_badarg() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::is_not_binary(arc_process.clone()),
                    strategy::term::is_encoding(),
                ),
                |(binary, encoding)| {
                    prop_assert_badarg!(
                        native(binary, encoding),
                        format!("binary ({}) must be a binary", binary)
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_binary_without_atom_encoding_errors_badarg() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::is_binary(arc_process.clone()),
                    strategy::term::is_not_atom(arc_process),
                ),
                |(binary, encoding)| {
                    prop_assert_badarg!(
                        native(binary, encoding),
                        format!("invalid encoding name value: `{}` is not an atom", encoding)
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_binary_with_atom_without_name_encoding_errors_badarg() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::is_binary(arc_process.clone()),
                    strategy::term::atom::is_not_encoding(),
                ),
                |(binary, encoding)| {
                    let encoding_atom: Atom = encoding.try_into().unwrap();

                    prop_assert_badarg!(
                        native(binary, encoding),
                        format!("invalid atom encoding name: '{0}' is not one of the supported values (latin1, unicode, or utf8)", encoding_atom.name())
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_utf8_binary_with_encoding_returns_atom_with_binary_name() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::binary::is_utf8(arc_process.clone()),
                    strategy::term::is_encoding(),
                ),
                |(binary, encoding)| {
                    let byte_vec: Vec<u8> = match binary.decode().unwrap() {
                        TypedTerm::HeapBinary(heap_binary) => heap_binary.as_bytes().to_vec(),
                        TypedTerm::SubBinary(subbinary) => subbinary.full_byte_iter().collect(),
                        TypedTerm::ProcBin(process_binary) => process_binary.as_bytes().to_vec(),
                        TypedTerm::BinaryLiteral(process_binary) => {
                            process_binary.as_bytes().to_vec()
                        }
                        typed_term => panic!("typed_term = {:?}", typed_term),
                    };

                    let s = std::str::from_utf8(&byte_vec).unwrap();

                    prop_assert_eq!(native(binary, encoding), Ok(Atom::str_to_term(s)));

                    Ok(())
                },
            )
            .unwrap();
    });
}
