use criterion::criterion_main;

use aoc_benchmarking::aoc_benches;
use calorie_counting::CalorieCounting;
use camp_cleanup::CampCleanup;
use rock_paper_scissors::RockPaperScissors;
use rucksack_reorganization::RucksackReorganization;
use supply_stacks::SupplyStacks;
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
    (
        day_003,
        "../day-003-rucksack-reorganization/input.txt",
        RucksackReorganization,
        "Part 1",
        "Part 2"
    ),
    (
        day_004,
        "../day-004-camp-cleanup/input.txt",
        CampCleanup,
        "Part 1",
        "Part 2"
    ),
    (
        day_005,
        "../day-005-supply-stacks/input.txt",
        SupplyStacks,
        "Part 1",
        "Part 2"
    ),
    // bench_marker
}
