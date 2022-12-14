use criterion::criterion_main;

use aoc_benchmarking::aoc_benches;
use beacon_exclusion_zone::BeaconExclusionZone;
use blizzard_basin::BlizzardBasin;
use boiling_boulders::BoilingBoulders;
use calorie_counting::CalorieCounting;
use camp_cleanup::CampCleanup;
use cathode_ray_tube::CathodeRayTube;
use distress_signal::DistressSignal;
use full_of_hot_air::FullOfHotAir;
use grove_positioning_system::GrovePositioningSystem;
use hill_climbing_algorithm::HillClimbingAlgorithm;
use monkey_in_the_middle::MonkeyInTheMiddle;
use monkey_map::MonkeyMap;
use monkey_math::MonkeyMath;
use no_space_left_on_device::NoSpaceLeftOnDevice;
use not_enough_minerals::NotEnoughMinerals;
use proboscidea_volcanium::ProboscideaVolcanium;
use pyroclastic_flow::PyroclasticFlow;
use regolith_reservoir::RegolithReservoir;
use rock_paper_scissors::RockPaperScissors;
use rope_bridge::RopeBridge;
use rucksack_reorganization::RucksackReorganization;
use supply_stacks::SupplyStacks;
use treetop_tree_house::TreetopTreeHouse;
use tuning_trouble::TuningTrouble;
use unstable_diffusion::UnstableDiffusion;
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
    (
        day_013,
        "../day-013-distress-signal/input.txt",
        DistressSignal,
        "Part 1",
        "Part 2"
    ),
    (
        day_014,
        "../day-014-regolith-reservoir/input.txt",
        RegolithReservoir,
        "Part 1",
        "Part 2"
    ),
    (
        day_015,
        "../day-015-beacon-exclusion-zone/input.txt",
        BeaconExclusionZone,
        "Part 1",
        "Part 2"
    ),
    (
        day_016,
        "../day-016-proboscidea-volcanium/input.txt",
        ProboscideaVolcanium,
        "Part 1",
        "Part 2"
    ),
    (
        day_017,
        "../day-017-pyroclastic-flow/input.txt",
        PyroclasticFlow,
        "Part 1",
        "Part 2"
    ),
    (
        day_018,
        "../day-018-boiling-boulders/input.txt",
        BoilingBoulders,
        "Part 1",
        "Part 2"
    ),
    (
        day_019,
        "../day-019-not-enough-minerals/input.txt",
        NotEnoughMinerals,
        "Part 1",
        "Part 2"
    ),
    (
        day_020,
        "../day-020-grove-positioning-system/input.txt",
        GrovePositioningSystem,
        "Part 1",
        "Part 2"
    ),
    (
        day_021,
        "../day-021-monkey-math/input.txt",
        MonkeyMath,
        "Part 1",
        "Part 2"
    ),
    (
        day_022,
        "../day-022-monkey-map/input.txt",
        MonkeyMap,
        "Part 1",
        "Part 2"
    ),
    (
        day_023,
        "../day-023-unstable-diffusion/input.txt",
        UnstableDiffusion,
        "Part 1",
        "Part 2"
    ),
    (
        day_024,
        "../day-024-blizzard-basin/input.txt",
        BlizzardBasin,
        "Part 1",
        "Part 2"
    ),
    (
        day_025,
        "../day-025-full-of-hot-air/input.txt",
        FullOfHotAir,
        "Part 1",
        "Part 2"
    ),
    // bench_marker
}
