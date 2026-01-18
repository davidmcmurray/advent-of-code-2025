fn main() {
    let puzzle: &'static str = advent_of_code_2025::load_puzzle_string(1).leak();
    let qty_positions_on_dial = 100;

    let mut current_dial_position = 50_i32;
    let mut p1_count_of_rotations_landing_on_zero = 0;
    for line in puzzle.lines() {
        let rotation = parse_dial_rotation(line)
            .expect("all lines in the input should be L or R, followed by a number");
        current_dial_position += rotation;
        current_dial_position = current_dial_position.rem_euclid(qty_positions_on_dial);
        if current_dial_position == 0 {
            p1_count_of_rotations_landing_on_zero += 1;
        }
    }
    dbg!(p1_count_of_rotations_landing_on_zero);
}

///
fn parse_dial_rotation(line: &str) -> Result<i32, &str> {
    let too_short = line.len() < 2;
    if too_short {
        return Err(line);
    }
    let direction = line.as_bytes()[0];
    let positive_or_negative = match direction {
        b'L' => -1,
        b'R' => 1,
        _ => return Err(line),
    };
    let number_text = &line[1..];
    let Ok(absolute_distance_rotated) = number_text.parse::<i32>() else {
        return Err(line);
    };
    let rotation = positive_or_negative * absolute_distance_rotated;
    Ok(rotation)
}
