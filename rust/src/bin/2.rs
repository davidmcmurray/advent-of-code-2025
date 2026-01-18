use std::{fmt::Write, io::BufRead};
fn main() {
    let puzzle_text = advent_of_code_2025::load_puzzle_string(2);

    let mut p1_invalid_count = 0;
    for (low, high) in parse_d2_ranges(&puzzle_text) {
        p1_invalid_count += sum_invalid_p1_ids_in_range(low, high)
    }
    dbg!(p1_invalid_count);
}
fn parse_d2_ranges(puzzle: &str) -> impl Iterator<Item = (u64, u64)> {
    puzzle
        .trim()
        .split(',')
        .filter(|comma_sep| !comma_sep.is_empty())
        .map(|text| {
            let (low_t, high_t) = text.split_once('-').unwrap();
            let low = low_t.parse().unwrap();
            let high = high_t.parse().unwrap();
            (low, high)
        })
}
/// Check if the number is some sequence of digits repeated twice.
fn id_is_invalid_p1(num: u64, buffer: &mut String) -> bool {
    // There are certainly ways to do this without touching memory, with ilog...
    // but converting the number to a string makes things simple.
    // I'll reuse a string instead of creating new ones, to keep performance reasonable.
    buffer.clear();
    _ = write!(buffer, "{num}");
    let half_len = buffer.len() / 2;
    let even_len = buffer.len() == half_len * 2;
    if !even_len {
        return false;
    }
    let (first_half, second_half) = buffer.as_bytes().split_at(half_len);
    first_half == second_half
}
fn sum_invalid_p1_ids_in_range(start: u64, end: u64) -> u64 {
    let mut reusable_string = String::new();
    (start..=end)
        .map(|n| {
            id_is_invalid_p1(n, &mut reusable_string)
                .then_some(n)
                .unwrap_or_default()
        })
        .sum()
}
/// If the id is a repetition of digits, of any length, it's invalid and should return true. Otherwise false.
fn id_is_invalid_p2(num: u64, buffer: &mut String) -> bool {
    buffer.clear();
    _ = write!(buffer, "{num}");
    false
}
fn sum_invalid_p2_ids_in_range(start: u64, end: u64) -> u64 {
    let mut reusable_string = String::new();
    (start..=end)
        .map(|n| {
            id_is_invalid_p2(n, &mut reusable_string)
                .then_some(n)
                .unwrap_or_default()
        })
        .sum()
}
