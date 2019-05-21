use super::*;

use num_traits::Num;

use crate::process::IntoProcess;

#[test]
fn with_atom_returns_bad_argument() {
    let mut process: Process = Default::default();
    let atom_term = Term::str_to_atom("atom", Existence::DoNotCare, &mut process).unwrap();

    assert_bad_argument!(erlang::binary_to_list(atom_term, &mut process), process);
}

#[test]
fn with_empty_list_is_bad_argument() {
    let mut process: Process = Default::default();

    assert_bad_argument!(
        erlang::binary_to_list(Term::EMPTY_LIST, &mut process),
        process
    );
}

#[test]
fn with_list_is_bad_argument() {
    let mut process: Process = Default::default();
    let list_term = list_term(&mut process);

    assert_bad_argument!(erlang::binary_to_list(list_term, &mut process), process);
}

#[test]
fn with_small_integer_is_bad_argument() {
    let mut process: Process = Default::default();
    let small_integer_term = 0usize.into_process(&mut process);

    assert_bad_argument!(
        erlang::binary_to_list(small_integer_term, &mut process),
        process
    );
}

#[test]
fn with_big_integer_is_bad_argument() {
    let mut process: Process = Default::default();
    let big_integer_term = <BigInt as Num>::from_str_radix("576460752303423489", 10)
        .unwrap()
        .into_process(&mut process);

    assert_bad_argument!(
        erlang::binary_to_list(big_integer_term, &mut process),
        process
    );
}

#[test]
fn with_float_is_bad_argument() {
    let mut process: Process = Default::default();
    let float_term = 1.0.into_process(&mut process);

    assert_bad_argument!(erlang::binary_to_list(float_term, &mut process), process);
}

#[test]
fn with_local_pid_is_bad_argument() {
    let mut process: Process = Default::default();
    let local_pid_term = Term::local_pid(0, 0).unwrap();

    assert_bad_argument!(
        erlang::binary_to_list(local_pid_term, &mut process),
        process
    );
}

#[test]
fn with_external_pid_is_bad_argument() {
    let mut process: Process = Default::default();
    let external_pid_term = Term::external_pid(1, 0, 0, &mut process).unwrap();

    assert_bad_argument!(
        erlang::binary_to_list(external_pid_term, &mut process),
        process
    );
}

#[test]
fn with_tuple_is_bad_argument() {
    let mut process: Process = Default::default();
    let tuple_term = Term::slice_to_tuple(&[], &mut process);

    assert_bad_argument!(erlang::binary_to_list(tuple_term, &mut process), process);
}

#[test]
fn with_heap_binary_returns_list_of_bytes() {
    let mut process: Process = Default::default();
    let heap_binary_term = Term::slice_to_binary(&[0, 1, 2], &mut process);

    assert_eq_in_process!(
        erlang::binary_to_list(heap_binary_term, &mut process),
        Ok(Term::cons(
            0.into_process(&mut process),
            Term::cons(
                1.into_process(&mut process),
                Term::cons(2.into_process(&mut process), Term::EMPTY_LIST, &mut process),
                &mut process
            ),
            &mut process
        )),
        process
    );
}

#[test]
fn with_subbinary_without_bit_count_returns_list_of_bytes() {
    let mut process: Process = Default::default();
    // <<1::1, 0, 1, 2>>
    let binary_term = Term::slice_to_binary(&[128, 0, 129, 0b0000_0000], &mut process);
    let subbinary_term = Term::subbinary(binary_term, 0, 1, 3, 0, &mut process);

    assert_eq_in_process!(
        erlang::binary_to_list(subbinary_term, &mut process),
        Ok(Term::cons(
            0.into_process(&mut process),
            Term::cons(
                1.into_process(&mut process),
                Term::cons(2.into_process(&mut process), Term::EMPTY_LIST, &mut process),
                &mut process
            ),
            &mut process
        )),
        process
    );
}

#[test]
fn with_subbinary_with_bit_count_returns_bad_argument() {
    let mut process: Process = Default::default();
    // <<1::1, 0, 1, 2>>
    let binary_term = Term::slice_to_binary(&[128, 0, 129, 0b0000_0000], &mut process);
    let subbinary_term = Term::subbinary(binary_term, 0, 0, 3, 1, &mut process);

    assert_bad_argument!(
        erlang::binary_to_list(subbinary_term, &mut process),
        process
    );
}