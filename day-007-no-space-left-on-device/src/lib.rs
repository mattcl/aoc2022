use std::str::FromStr;

use anyhow::{anyhow, bail};
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum History<'a> {
    Cd { path: &'a str },
    Ls,
    File { size: u64, name: &'a str },
    Dir { name: &'a str },
}

fn parse_cd<'a>(input: &'a str) -> IResult<&'a str, History<'a>> {
    let (input, name) = preceded(tag("$ cd "), rest)(input)?;
    Ok((input, History::Cd { path: name }))
}

fn parse_ls<'a>(input: &'a str) -> IResult<&'a str, History<'a>> {
    let (input, _) = tag("$ ls")(input)?;
    Ok((input, History::Ls))
}

fn parse_file<'a>(input: &'a str) -> IResult<&'a str, History<'a>> {
    let (input, (size, name)) = separated_pair(complete::u64, tag(" "), rest)(input)?;
    Ok((input, History::File { size, name }))
}

fn parse_dir<'a>(input: &'a str) -> IResult<&'a str, History<'a>> {
    let (input, name) = preceded(tag("dir "), rest)(input)?;
    Ok((input, History::Dir { name }))
}

fn parse_history<'a>(input: &'a str) -> IResult<&'a str, History<'a>> {
    alt((parse_ls, parse_cd, parse_dir, parse_file))(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Inode {
    File {
        inode: usize,
        size: u64,
        parent: usize,
    },
    Dir {
        inode: usize,
        entries: FxHashMap<String, usize>,
        parent: usize,
    },
}

impl Inode {
    pub fn parent(&self) -> usize {
        match self {
            Self::File { parent, .. } => *parent,
            Self::Dir { parent, .. } => *parent,
        }
    }

    pub fn inode(&self) -> usize {
        match self {
            Self::File { inode, .. } => *inode,
            Self::Dir { inode, .. } => *inode,
        }
    }

    pub fn size(&self, inodes: &[Inode], cache: &mut FxHashMap<usize, u64>) -> u64 {
        match self {
            Self::File { size, .. } => *size,
            Self::Dir { inode, entries, .. } => {
                if let Some(s) = cache.get(inode) {
                    *s
                } else {
                    let s = entries
                        .values()
                        .map(|i| inodes[*i].size(inodes, cache))
                        .sum();
                    cache.insert(*inode, s);
                    s
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct NoSpaceLeftOnDevice {
    inodes: Vec<Inode>,
}

impl FromStr for NoSpaceLeftOnDevice {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut filesystem = Self::default();

        filesystem.inodes.push(Inode::Dir {
            inode: 0,
            entries: FxHashMap::default(),
            parent: 0,
        });

        let mut cur = 0;

        for res in s.trim().lines().map(|l| parse_history(l.trim())) {
            let (_, out) = res.map_err(|e| e.to_owned())?;

            let next_inode = filesystem.inodes.len();
            match out {
                History::File { size, name } => {
                    filesystem.inodes.push(Inode::File {
                        inode: next_inode,
                        size,
                        parent: filesystem.inodes[cur].inode(),
                    });
                    match &mut filesystem.inodes[cur] {
                        Inode::Dir { entries, .. } => entries.insert(name.into(), next_inode),
                        _ => bail!("attempted to insert entry to a file"),
                    };
                }
                History::Dir { name } => {
                    filesystem.inodes.push(Inode::Dir {
                        inode: next_inode,
                        entries: FxHashMap::default(),
                        parent: filesystem.inodes[cur].inode(),
                    });
                    match &mut filesystem.inodes[cur] {
                        Inode::Dir { entries, .. } => entries.insert(name.into(), next_inode),
                        _ => bail!("attempted to insert entry to a file"),
                    };
                }
                History::Cd { path } => {
                    if path == ".." {
                        cur = filesystem.inodes[cur].parent();
                    } else if path == "/" {
                        cur = 0;
                    } else {
                        cur = match filesystem.inodes.get(cur) {
                            Some(Inode::Dir { entries, .. }) => {
                                *entries.get(path).ok_or_else(|| {
                                    anyhow!("Attempted to get missing path: {}", path)
                                })?
                            }
                            _ => bail!("attempted to search file for entries"),
                        };
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
        let mut cache = FxHashMap::default();
        // warm the cache
        self.inodes[0].size(&self.inodes, &mut cache);
        Ok(cache.values().filter(|v| **v <= 100000).sum())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        let mut cache = FxHashMap::default();
        // warm the cache
        let cur = 70000000 - self.inodes[0].size(&self.inodes, &mut cache);
        let desired = 30000000 - cur;

        cache
            .values()
            .filter(|v| **v >= desired)
            .min()
            .map(|v| *v)
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
