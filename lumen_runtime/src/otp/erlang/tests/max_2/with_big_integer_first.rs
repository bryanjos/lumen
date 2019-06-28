use super::*;

#[test]
fn with_lesser_small_integer_second_returns_first() {
    max(|_, process| 0.into_process(&process), First)
}

#[test]
fn with_greater_small_integer_second_returns_second() {
    super::max(
        |process| (crate::integer::small::MIN - 1).into_process(&process),
        |_, process| crate::integer::small::MIN.into_process(&process),
        Second,
    );
}

#[test]
fn with_lesser_big_integer_second_returns_first() {
    max(
        |_, process| (crate::integer::small::MIN - 1).into_process(&process),
        First,
    )
}

#[test]
fn with_same_big_integer_second_returns_first() {
    max(|first, _| first, First)
}

#[test]
fn with_same_value_big_integer_second_returns_first() {
    max(
        |_, process| (crate::integer::small::MAX + 1).into_process(&process),
        First,
    )
}

#[test]
fn with_greater_big_integer_second_returns_second() {
    max(
        |_, process| (crate::integer::small::MAX + 2).into_process(&process),
        Second,
    )
}

#[test]
fn with_lesser_float_second_returns_first() {
    max(|_, process| 0.0.into_process(&process), First)
}

#[test]
fn with_greater_float_second_returns_second() {
    super::max(
        |process| (crate::integer::small::MIN - 1).into_process(&process),
        |_, process| 0.0.into_process(&process),
        Second,
    );
}

#[test]
fn without_second_number_returns_second() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::integer::big(arc_process.clone()),
                    strategy::term::is_not_number(arc_process.clone()),
                ),
                |(first, second)| {
                    prop_assert_eq!(erlang::max_2(first, second), second);

                    Ok(())
                },
            )
            .unwrap();
    });
}

fn max<R>(second: R, which: FirstSecond)
where
    R: FnOnce(Term, &Process) -> Term,
{
    super::max(
        |process| (crate::integer::small::MAX + 1).into_process(&process),
        second,
        which,
    );
}
