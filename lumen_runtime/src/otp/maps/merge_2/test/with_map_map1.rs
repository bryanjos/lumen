mod with_map_map2;

use super::*;

#[test]
fn without_map_map2_errors_badmap() {
    TestRunner::new(Config::with_source_file(file!()))
        .run(
            &strategy::process().prop_flat_map(|arc_process| {
                (
                    Just(arc_process.clone()),
                    strategy::term::is_map(arc_process.clone()),
                    strategy::term::is_not_map(arc_process.clone()),
                )
            }),
            |(arc_process, map1, map2)| {
                prop_assert_badmap!(
                    native(&arc_process, map1, map2),
                    &arc_process,
                    map2,
                    format!("map2 ({}) is not a map", map2)
                );

                Ok(())
            },
        )
        .unwrap();
}
