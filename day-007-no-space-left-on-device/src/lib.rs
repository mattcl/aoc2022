use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::rest,
    sequence::{preceded, separated_pair},
    IResult,
};
use rustc_hash::FxHashMap;
use xxhash_rust::xxh3::xxh3_64;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum History {
    Cd { path: u64 },
    Ls,
    File { size: u64 },
    Dir { name: u64 },
}

fn parse_cd(input: &str) -> IResult<&str, History> {
    let (input, name) = preceded(tag("$ cd "), rest)(input)?;
    Ok((
        input,
        History::Cd {
            path: xxh3_64(name.as_bytes()),
        },
    ))
}

fn parse_ls(input: &str) -> IResult<&str, History> {
    let (input, _) = tag("$ ls")(input)?;
    Ok((input, History::Ls))
}

fn parse_file(input: &str) -> IResult<&str, History> {
    let (input, (size, _)) = separated_pair(complete::u64, tag(" "), rest)(input)?;
    Ok((input, History::File { size }))
}

fn parse_dir(input: &str) -> IResult<&str, History> {
    let (input, name) = preceded(tag("dir "), rest)(input)?;
    Ok((
        input,
        History::Dir {
            name: xxh3_64(name.as_bytes()),
        },
    ))
}

fn parse_history(input: &str) -> IResult<&str, History> {
    alt((parse_ls, parse_cd, parse_dir, parse_file))(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Directory {
    inode: usize,
    directories: FxHashMap<u64, usize>,
    parent: usize,
    filesize: u64,
}

impl Directory {
    pub fn parent(&self) -> usize {
        self.parent
    }

    pub fn inode(&self) -> usize {
        self.inode
    }

    pub fn size(
        &self,
        directories: &[Directory],
        results: &mut Vec<u64>,
        criteria: impl Fn(u64) -> bool + Copy,
    ) -> u64 {
        let s = self.filesize
            + self
                .directories
                .values()
                .map(|i| directories[*i].size(directories, results, criteria))
                .sum::<u64>();

        if criteria(s) {
            results.push(s);
        }

        s
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct NoSpaceLeftOnDevice {
    directories: Vec<Directory>,
    total_size: u64,
}

impl FromStr for NoSpaceLeftOnDevice {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut filesystem = Self::default();

        filesystem.directories.push(Directory {
            inode: 0,
            directories: FxHashMap::default(),
            parent: 0,
            filesize: 0,
        });

        let up = xxh3_64("..".as_bytes());
        let root = xxh3_64("/".as_bytes());

        let mut cur = 0;

        for res in s.trim().lines().map(|l| parse_history(l.trim())) {
            let (_, out) = res.map_err(|e| e.to_owned())?;

            let next_inode = filesystem.directories.len();
            match out {
                History::File { size } => {
                    filesystem.total_size += size;
                    filesystem.directories[cur].filesize += size;
                }
                History::Dir { name } => {
                    filesystem.directories.push(Directory {
                        inode: next_inode,
                        directories: FxHashMap::default(),
                        parent: filesystem.directories[cur].inode(),
                        filesize: 0,
                    });
                    filesystem.directories[cur]
                        .directories
                        .insert(name, next_inode);
                }
                History::Cd { path } => {
                    if path == up {
                        cur = filesystem.directories[cur].parent();
                    } else if path == root {
                        cur = 0;
                    } else {
                        cur = *filesystem.directories[cur]
                            .directories
                            .get(&path)
                            .ok_or_else(|| {
                                anyhow!("Attempting to get unknown directory: {}", path)
                            })?;
                    }
                }
                History::Ls => { /* what does this even do? */ }
            }
        }

        Ok(filesystem)
    }
}

impl Problem for NoSpaceLeftOnDevice {
    const DAY: usize = 7;
    const TITLE: &'static str = "no space left on device";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u64;
    type P2 = u64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        let mut results = Vec::with_capacity(self.directories.len());
        self.directories[0].size(&self.directories, &mut results, |v| v <= 100000);
        Ok(results.iter().sum())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        let mut results = Vec::with_capacity(self.directories.len());
        let desired = 30000000 - (70000000 - self.total_size);
        self.directories[0].size(&self.directories, &mut results, |v| v >= desired);

        results
            .into_iter()
            .min()
            .ok_or_else(|| anyhow!("could not find directory"))
    }
}

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = NoSpaceLeftOnDevice::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1792222, 1112963));
    }

    #[test]
    fn example() {
        let input = "
            $ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k
            ";
        let solution = NoSpaceLeftOnDevice::solve(input).unwrap();
        assert_eq!(solution, Solution::new(95437, 24933642));
    }
}
