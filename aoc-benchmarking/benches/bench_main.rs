use criterion::criterion_main;

use aoc_benchmarking::aoc_benches;
use calorie_counting::CalorieCounting;
use rock_paper_scissors::RockPaperScissors;
// import_marker

criterion_main! {
    benches
}

aoc_benches! {
    5,
    (
        day_001,
        "../day-001-calorie-counting/input.txt",
        CalorieCounting,
        "Part 1",
        "Part 2"
    ),
    (
        day_002,
        "../day-002-rock-paper-scissors/input.txt",
        RockPaperScissors,
        "Part 1",
        "Part 2"
    ),
    // bench_marker
}
