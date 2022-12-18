use std::collections::HashMap;

use super::day::Day;
use anyhow::Result;

pub struct Day7;
impl Day for Day7 {
    type Parsed = HashMap<Vec<String>, i32>;
    type Output = i32;

    fn parse(input: String) -> Result<Self::Parsed> {
        let mut folders = HashMap::new();
        let mut current_folders: Vec<String> = vec![];
        for line in input.lines() {
            if let Some(folder) = line.strip_prefix("$ cd ") {
                if folder == ".." {
                    current_folders.pop();
                } else {
                    current_folders.push(folder.to_string());
                    folders.insert(current_folders.clone(), 0);
                }
            } else if line.chars().next().unwrap().is_numeric() {
                let mut own = current_folders.clone();
                while !own.is_empty() {
                    *folders.get_mut(&own).unwrap() +=
                        line.split_once(' ').unwrap().0.parse::<i32>()?;
                    own.pop();
                }
            }
        }
        Ok(folders)
    }
    fn first(folders: Self::Parsed) -> Self::Output {
        folders.iter().map(|(_, &size)| size).filter(|&size| size <= 100_000).sum::<i32>()
    }
    fn second(folders: Self::Parsed) -> Self::Output {
        let min_to_delete = folders.get(&vec!["/".to_string()]).unwrap() - 40_000_000;
        folders.iter().map(|(_, &size)| size).filter(|&size| size > min_to_delete).min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "$ cd /
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
7214296 k";
    fn parsed() -> <Day7 as Day>::Parsed {
        Day7::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day7::first(parsed()), 95437);
    }
    #[test]
    fn part2() {
        assert_eq!(Day7::second(parsed()), 24933642);
    }
}