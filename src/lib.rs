pub mod analysis;
pub mod error;
pub mod map;
pub mod markov;
pub mod navigation;
pub mod tui;

pub fn run(
    mut map: map::Map,
    slope: (usize, usize),
    termination: navigation::Termination,
    output: navigation::Output,
) -> String {
    let mut state = navigation::State::new();
    loop {
        if navigation::terminal(&termination, &map, &state) {
            break navigation::output(&output, (map, state));
        } else {
            navigation::step(&mut map, &mut state, slope);
        }
    }
}
