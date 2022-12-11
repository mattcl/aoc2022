use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};
use aoc_plumbing::Problem;
use calorie_counting::CalorieCounting;
use camp_cleanup::CampCleanup;
use cathode_ray_tube::CathodeRayTube;
use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, shells::Zsh};
use monkey_in_the_middle::MonkeyInTheMiddle;
use no_space_left_on_device::NoSpaceLeftOnDevice;
use rock_paper_scissors::RockPaperScissors;
use rope_bridge::RopeBridge;
use rucksack_reorganization::RucksackReorganization;
use supply_stacks::SupplyStacks;
use treetop_tree_house::TreetopTreeHouse;
use tuning_trouble::TuningTrouble;
// import_marker

// I'm not proud
macro_rules! generate_cli {
    ($(($name:ident, $day:literal)),* $(,)?) => {
        #[derive(Parser)]
        pub(crate) struct Cli {
            #[command(subcommand)]
            pub command: Commands,
        }

        impl Cli {
            pub fn run() -> Result<()> {
                let command = Self::parse().command;
                command.run()
            }
        }

        #[derive(Subcommand)]
        pub(crate) enum Commands {
            $(
            #[command(about = $name::problem_label(), long_about = $name::long_description(), display_order = $day)]
            $name(Solver<$name>),
            )*

            #[command(display_order = 30)]
            Run(Run),

            #[command(display_order = 31)]
            GenerateCompletions(GenerateCompletions),
        }

        impl Commands {
            pub fn run(&self) -> Result<()> {
                match self {
                    Self::GenerateCompletions(cmd) => cmd.run(),
                    Self::Run(cmd) => cmd.run(),
                    $(
                    Self::$name(cmd) => cmd.run(),
                    )*
                }
            }
        }

        /// Run the solution for a specified day.
        ///
        /// The day must be implemented and the specified input must exist.
        #[derive(Args)]
        pub(crate) struct Run {
            /// The day to run.
            day: usize,

            /// The path to the input for this solution.
            input: PathBuf,

            /// Display the output as json.
            #[clap(short, long)]
            json: bool,
        }

        impl Run {
            pub fn run(&self) -> Result<()> {
                match self.day {
                    $(
                    $day => _run::<$name>(&self.input, self.json),
                    )*
                    _ => Err(anyhow!("Unknown day: {}", self.day))
                }
            }
        }
    };
}

#[derive(Args)]
pub(crate) struct Solver<T>
where
    T: Problem,
{
    /// The path to the input for this solution.
    input: PathBuf,

    /// Display the output as json.
    #[clap(short, long)]
    json: bool,

    #[clap(skip)]
    _phantom: PhantomData<T>,
}

impl<T> Solver<T>
where
    T: Problem,
    <T as Problem>::ProblemError: Into<anyhow::Error>,
{
    pub fn run(&self) -> Result<()> {
        _run::<T>(&self.input, self.json)
    }
}

fn _run<T>(input_file: &Path, json: bool) -> Result<()>
where
    T: Problem,
    <T as Problem>::ProblemError: Into<anyhow::Error>,
{
    let input = std::fs::read_to_string(input_file).context("Could not read input file")?;

    let solution = T::solve(&input)
        .map_err(Into::<anyhow::Error>::into)
        .context("Failed to solve")?;

    if json {
        println!("{}", serde_json::to_string(&solution)?);
    } else {
        println!("{}", solution);
    }

    Ok(())
}

/// Generate zsh completions
#[derive(Debug, Args)]
pub struct GenerateCompletions;

impl GenerateCompletions {
    fn run(&self) -> Result<()> {
        generate(Zsh, &mut Cli::command(), "aoc", &mut std::io::stdout());
        Ok(())
    }
}

generate_cli! {
    (CalorieCounting, 1),
    (RockPaperScissors, 2),
    (RucksackReorganization, 3),
    (CampCleanup, 4),
    (SupplyStacks, 5),
    (TuningTrouble, 6),
    (NoSpaceLeftOnDevice, 7),
    (TreetopTreeHouse, 8),
    (RopeBridge, 9),
    (CathodeRayTube, 10),
    (MonkeyInTheMiddle, 11),
    // command_marker
}
