use criterion::criterion_main;

use aoc_benchmarking::aoc_benches;
use calorie_counting::CalorieCounting;
use camp_cleanup::CampCleanup;
use cathode_ray_tube::CathodeRayTube;
use hill_climbing_algorithm::HillClimbingAlgorithm;
use monkey_in_the_middle::MonkeyInTheMiddle;
use no_space_left_on_device::NoSpaceLeftOnDevice;
use rock_paper_scissors::RockPaperScissors;
use rope_bridge::RopeBridge;
use rucksack_reorganization::RucksackReorganization;
use supply_stacks::SupplyStacks;
use treetop_tree_house::TreetopTreeHouse;
use tuning_trouble::TuningTrouble;
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
    (
        day_006,
        "../day-006-tuning-trouble/input.txt",
        TuningTrouble,
        "Part 1",
        "Part 2"
    ),
    (
        day_007,
        "../day-007-no-space-left-on-device/input.txt",
        NoSpaceLeftOnDevice,
        "Part 1",
        "Part 2"
    ),
    (
        day_008,
        "../day-008-treetop-tree-house/input.txt",
        TreetopTreeHouse,
        "Combined because of parts being linked (includes parsing)"
    ),
    (
        day_009,
        "../day-009-rope-bridge/input.txt",
        RopeBridge,
        "Part 1",
        "Part 2"
    ),
    (
        day_010,
        "../day-010-cathode-ray-tube/input.txt",
        CathodeRayTube,
        "Part 1",
        "Part 2"
    ),
    (
        day_011,
        "../day-011-monkey-in-the-middle/input.txt",
        MonkeyInTheMiddle,
        "Part 1",
        "Part 2"
    ),
    (
        day_012,
        "../day-012-hill-climbing-algorithm/input.txt",
        HillClimbingAlgorithm,
        "Part 1",
        "Part 2"
    ),
    // bench_marker
}
