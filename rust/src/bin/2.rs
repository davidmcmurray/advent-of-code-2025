fn main() {
    let puzzle_text = advent_of_code_2025::load_puzzle_string(2);

    let mut p1_invalid_count = 0;
    for (low, high) in parse_d2_ranges(&puzzle_text) {}
}
fn parse_d2_ranges(puzzle: &str) -> impl Iterator<Item = (u64, u64)> {
    puzzle
        .split(',')
        .filter(|comma_sep| !comma_sep.is_empty())
        .map(|text| {
            let (low_t, high_t) = text.split_once('-').unwrap();
            let low = low_t.parse().unwrap();
            let high = high_t.parse().unwrap();
            (low, high)
        })
}
