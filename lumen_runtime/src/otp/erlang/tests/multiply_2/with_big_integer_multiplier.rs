use super::*;

#[test]
fn with_atom_multiplicand_errors_badarith() {
    with_multiplicand_errors_badarith(|_| Term::str_to_atom("multiplier", DoNotCare).unwrap());
}

#[test]
fn with_local_reference_multiplicand_errors_badarith() {
    with_multiplicand_errors_badarith(|mut process| Term::local_reference(&mut process));
}

#[test]
fn with_empty_list_multiplicand_errors_badarith() {
    with_multiplicand_errors_badarith(|_| Term::EMPTY_LIST);
}

#[test]
fn with_list_multiplicand_errors_badarith() {
    with_multiplicand_errors_badarith(|mut process| {
        Term::cons(
            0.into_process(&mut process),
            1.into_process(&mut process),
            &mut process,
        )
    });
}

#[test]
fn with_small_integer_multiplicand_with_returns_big_integer() {
    with(|multiplier, mut process| {
        let multiplicand = crate::integer::small::MIN.into_process(&mut process);

        assert_eq!(multiplicand.tag(), SmallInteger);

        let result = erlang::multiply_2(multiplier, multiplicand, &mut process);

        assert!(result.is_ok());

        let product = result.unwrap();

        assert_eq!(product.tag(), Boxed);

        let unboxed_product: &Term = product.unbox_reference();

        assert_eq!(unboxed_product.tag(), BigInteger);
    })
}

#[test]
fn with_big_integer_multiplicand_returns_big_integer() {
    with(|multiplier, mut process| {
        let multiplicand = (crate::integer::small::MAX + 1).into_process(&mut process);

        assert_eq!(multiplicand.tag(), Boxed);

        let unboxed_multiplicand: &Term = multiplicand.unbox_reference();

        assert_eq!(unboxed_multiplicand.tag(), BigInteger);

        let result = erlang::multiply_2(multiplier, multiplicand, &mut process);

        assert!(result.is_ok());

        let product = result.unwrap();

        assert_eq!(product.tag(), Boxed);

        let unboxed_product: &Term = product.unbox_reference();

        assert_eq!(unboxed_product.tag(), BigInteger);
    })
}

#[test]
fn with_float_multiplicand_without_underflow_or_overflow_returns_float() {
    with(|multiplier, mut process| {
        let multiplicand = 3.0.into_process(&mut process);

        let result = erlang::multiply_2(multiplier, multiplicand, &mut process);

        assert!(result.is_ok());

        let product = result.unwrap();

        assert_eq!(product.tag(), Boxed);

        let unboxed_product: &Term = product.unbox_reference();

        assert_eq!(unboxed_product.tag(), Float);
    })
}

#[test]
fn with_float_multiplicand_with_underflow_returns_min_float() {
    with(|multiplier, mut process| {
        let multiplicand = std::f64::MIN.into_process(&mut process);

        assert_eq!(
            erlang::multiply_2(multiplier, multiplicand, &mut process),
            Ok(std::f64::MIN.into_process(&mut process))
        );
    })
}

#[test]
fn with_float_multiplicand_with_overflow_returns_max_float() {
    with(|multiplier, mut process| {
        let multiplicand = std::f64::MAX.into_process(&mut process);

        assert_eq!(
            erlang::multiply_2(multiplier, multiplicand, &mut process),
            Ok(std::f64::MAX.into_process(&mut process))
        );
    })
}

#[test]
fn with_local_pid_multiplicand_errors_badarith() {
    with_multiplicand_errors_badarith(|_| Term::local_pid(0, 1).unwrap());
}

#[test]
fn with_external_pid_multiplicand_errors_badarith() {
    with_multiplicand_errors_badarith(|mut process| {
        Term::external_pid(1, 2, 3, &mut process).unwrap()
    });
}

#[test]
fn with_tuple_multiplicand_errors_badarith() {
    with_multiplicand_errors_badarith(|mut process| Term::slice_to_tuple(&[], &mut process));
}

#[test]
fn with_map_is_multiplicand_errors_badarith() {
    with_multiplicand_errors_badarith(|mut process| Term::slice_to_map(&[], &mut process));
}

#[test]
fn with_heap_binary_multiplicand_errors_badarith() {
    with_multiplicand_errors_badarith(|mut process| Term::slice_to_binary(&[], &mut process));
}

#[test]
fn with_subbinary_multiplicand_errors_badarith() {
    with_multiplicand_errors_badarith(|mut process| {
        let original =
            Term::slice_to_binary(&[0b0000_00001, 0b1111_1110, 0b1010_1011], &mut process);
        Term::subbinary(original, 0, 7, 2, 1, &mut process)
    });
}

fn with<F>(f: F)
where
    F: FnOnce(Term, &mut Process) -> (),
{
    with_process(|mut process| {
        let multiplier: Term = (crate::integer::small::MAX + 1).into_process(&mut process);

        assert_eq!(multiplier.tag(), Boxed);

        let unboxed_multiplier: &Term = multiplier.unbox_reference();

        assert_eq!(unboxed_multiplier.tag(), BigInteger);

        f(multiplier, &mut process)
    })
}

fn with_multiplicand_errors_badarith<M>(multiplicand: M)
where
    M: FnOnce(&mut Process) -> Term,
{
    super::errors_badarith(|mut process| {
        let multiplier: Term = (crate::integer::small::MAX + 1).into_process(&mut process);

        assert_eq!(multiplier.tag(), Boxed);

        let unboxed_multiplier: &Term = multiplier.unbox_reference();

        assert_eq!(unboxed_multiplier.tag(), BigInteger);

        let multiplicand = multiplicand(&mut process);

        erlang::multiply_2(multiplier, multiplicand, &mut process)
    });
}