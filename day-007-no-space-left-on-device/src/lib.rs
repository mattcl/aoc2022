use std::{path::PathBuf, str::FromStr};

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Output {
    Cd { path: String },
    Ls,
    File { size: u64, name: String },
    Dir { name: String },
}

fn parse_cd(input: &str) -> IResult<&str, Output> {
    let (input, name) = preceded(tag("$ cd "), rest)(input)?;
    Ok((input, Output::Cd { path: name.into() }))
}

fn parse_ls(input: &str) -> IResult<&str, Output> {
    let (input, _) = tag("$ ls")(input)?;
    Ok((input, Output::Ls))
}

fn parse_file(input: &str) -> IResult<&str, Output> {
    let (input, (size, name)) = separated_pair(complete::u64, tag(" "), rest)(input)?;
    Ok((
        input,
        Output::File {
            size,
            name: name.into(),
        },
    ))
}

fn parse_dir(input: &str) -> IResult<&str, Output> {
    let (input, name) = preceded(tag("dir "), rest)(input)?;
    Ok((input, Output::Dir { name: name.into() }))
}

fn parse_output(input: &str) -> IResult<&str, Output> {
    alt((parse_cd, parse_ls, parse_dir, parse_file))(input)
}

impl FromStr for Output {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, o) = parse_output(s).map_err(|e| e.to_owned())?;
        Ok(o)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Inode {
    File {
        inode: usize,
        size: u64,
        name: String,
        parent: usize,
    },
    Dir {
        inode: usize,
        name: String,
        entries: Vec<usize>,
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
                    let s = entries.iter().map(|i| inodes[*i].size(inodes, cache)).sum();
                    cache.insert(*inode, s);
                    s
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct NoSpaceLeftOnDevice {
    inode_map: FxHashMap<PathBuf, usize>,
    inodes: Vec<Inode>,
}

impl FromStr for NoSpaceLeftOnDevice {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nslod = Self::default();

        nslod.inodes.push(Inode::Dir {
            inode: 0,
            name: "/".into(),
            entries: Vec::default(),
            parent: 0,
        });
        nslod.inode_map.insert("/".into(), 0);
        let mut cur_path = PathBuf::from("/");
        let mut cur = 0;

        for res in s.trim().lines().map(|l| Output::from_str(l.trim())) {
            let out = res?;

            let next_inode = nslod.inodes.len();
            match out {
                Output::File { size, name } => {
                    cur_path.push(&name);
                    nslod.inode_map.insert(cur_path.clone(), next_inode);
                    nslod.inodes.push(Inode::File {
                        inode: next_inode,
                        size,
                        name,
                        parent: nslod.inodes[cur].inode(),
                    });
                    match &mut nslod.inodes[cur] {
                        Inode::Dir { entries, .. } => entries.push(next_inode),
                        _ => bail!("attempted to insert entry to a file"),
                    }
                    cur_path.pop();
                }
                Output::Dir { name } => {
                    cur_path.push(&name);
                    nslod.inode_map.insert(cur_path.clone(), next_inode);
                    nslod.inodes.push(Inode::Dir {
                        inode: next_inode,
                        name,
                        entries: Vec::default(),
                        parent: nslod.inodes[cur].inode(),
                    });
                    match &mut nslod.inodes[cur] {
                        Inode::Dir { entries, .. } => entries.push(next_inode),
                        _ => bail!("attempted to insert entry to a file"),
                    }
                    cur_path.pop();
                }
                Output::Cd { path } => {
                    if path == ".." {
                        cur = nslod.inodes[cur].parent();
                        cur_path.pop();
                    } else if path == "/" {
                        cur = 0;
                        cur_path = PathBuf::from("/");
                    } else {
                        cur_path.push(path);
                        cur = nslod.inodes[*nslod
                            .inode_map
                            .get(&cur_path)
                            .ok_or_else(|| anyhow!("Unkonwn path: {:?}", &cur_path))?]
                        .inode();
                    }
                }
                Output::Ls => { /* what does this even do? */ }
            }
        }

        Ok(nslod)
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
        let sum = self
            .inodes
            .iter()
            .filter_map(|i| match i {
                Inode::Dir { .. } => Some(i.size(&self.inodes, &mut cache)),
                _ => None,
            })
            .filter(|v| *v <= 100000)
            .sum();

        Ok(sum)
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
