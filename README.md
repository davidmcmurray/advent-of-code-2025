# Advent of Code 2025

Solutions to 2025's Advent of Code puzzles as I work through them fashionably late. Most days will probably be done in Rust. My priority for this year's puzzles is to uphold readability. See below for commentary on my approaches, the puzzles themselves

<details>
    <summary>Day 1 Part 1</summary>
    Looks like a straightforward simulation to start. One could even do the ticks one-by-one, but a little bit of modular arithmetic makes that unnecessary. Rotations are negative for left, positive for right. New dial position = (old dial position + rotation) mod 100.
</details>

<details>
    <summary>Day 1 Part 2</summary>
    The easiest way to count how many times zero is passed would certainly be to do the ticks one at a time, but I'm still interested in avoiding that. Counting each full turn in a rotation (e.g. there are 2 full turns in L203), plus each time the dial position variable crosses out of bounds when adding the remainder (e.g. L203 -> handle the 2 turns -> -3 remainder, could cross zero if dial's at 1), plus each time it lands on zero as in part 1, should handle everything. The tricky part was avoiding an off-by-one when starting from zero and rotating left.
</details>