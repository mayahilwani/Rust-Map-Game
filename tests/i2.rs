use cleanav::{map::Map, navigation, run};

#[test]
fn i2_test_symbolcount_step1() {
    let steps = 15;
    let map = Map::from_path("./maps/basic.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Steps(steps),
            navigation::Output::SymbolCount(navigation::Cell::Trash)
        ),
        format!("{}", 1)
    );
}
