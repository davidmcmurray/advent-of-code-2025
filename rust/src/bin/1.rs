fn main() {
    let puzzle: &'static str = advent_of_code_2025::load_puzzle_string(1).leak();
    let qty_positions_on_dial = 100;

    let mut current_dial_position = 50_i32;
    let mut p1_count_of_rotations_landing_on_zero = 0;
    let mut p2_count_of_times_zero_is_touched = 0;

    for line in puzzle.lines() {
        let rotation = parse_dial_rotation(line)
            .expect("all lines in the input should be L or R, followed by a number");

        let started_on_zero = current_dial_position == 0;
        let full_turns = rotation.abs() / qty_positions_on_dial;

        p2_count_of_times_zero_is_touched += full_turns;

        let rotation_remainder = rotation % qty_positions_on_dial;
        current_dial_position += rotation_remainder;

        let partial_turn_crossed_zero = !started_on_zero && current_dial_position < 0
            || qty_positions_on_dial < current_dial_position;
        if partial_turn_crossed_zero {
            p2_count_of_times_zero_is_touched += 1;
        }

        current_dial_position = current_dial_position.rem_euclid(qty_positions_on_dial);

        let landed_exactly_on_zero = current_dial_position == 0;
        if landed_exactly_on_zero {
            p1_count_of_rotations_landing_on_zero += 1;
            p2_count_of_times_zero_is_touched += 1;
        }
    }
    dbg!(p1_count_of_rotations_landing_on_zero);
    dbg!(p2_count_of_times_zero_is_touched);
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
