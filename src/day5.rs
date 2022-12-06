use std::{cmp::min, collections::VecDeque, str::Split};

use crate::utils::must_read_file;

pub fn part1() {
    let data = must_read_file("data/day5/part1.txt");
    let mut lines = data.split("\n");
    let mut crate_stacks = Vec::new();
    parse_crate_stacks(&mut crate_stacks, &mut lines);

    let total_stacks = crate_stacks.len();
    lines
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let words = line.split(" ").collect::<Vec<_>>();
            if words.len() != 6 {
                return;
            }

            let from_stack = words[3].parse::<usize>().ok().unwrap() - 1;
            let to_stack = words[5].parse::<usize>().ok().unwrap() - 1;
            let count = min(
                words[1].parse::<usize>().unwrap(),
                crate_stacks[from_stack].len(),
            );

            if from_stack >= total_stacks || to_stack >= total_stacks {
                return;
            }

            (0..count).for_each(|_| {
                let value = crate_stacks[from_stack].pop_back().unwrap();
                crate_stacks[to_stack].push_back(value);
            });
        });

    print!("Top of stacks: ");
    for mut stack in crate_stacks {
        if let Some(value) = stack.pop_back() {
            print!("{value}");
        } else {
            print!(" ")
        }
    }
    println!();
}

pub fn part2() {
    let data = must_read_file("data/day5/part2.txt");
    let mut lines = data.split("\n");
    let mut crate_stacks = Vec::new();
    parse_crate_stacks(&mut crate_stacks, &mut lines);

    let total_stacks = crate_stacks.len();
    lines
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let words = line.split(" ").collect::<Vec<_>>();
            if words.len() != 6 {
                return;
            }

            let from_stack = words[3].parse::<usize>().ok().unwrap() - 1;
            let to_stack = words[5].parse::<usize>().ok().unwrap() - 1;
            let count = min(
                words[1].parse::<usize>().unwrap(),
                crate_stacks[from_stack].len(),
            );

            if from_stack >= total_stacks || to_stack >= total_stacks {
                return;
            }

            let mut removed = VecDeque::new();
            (0..count).for_each(|_| {
                let value = crate_stacks[from_stack].pop_back().unwrap();
                removed.push_front(value);
            });

            removed.iter().for_each(|value| {
                crate_stacks[to_stack].push_back(*value);
            });
        });

    print!("Top of stacks: ");
    for mut stack in crate_stacks {
        if let Some(value) = stack.pop_back() {
            print!("{value}");
        } else {
            print!(" ")
        }
    }
    println!();
}

fn parse_crate_stacks(crate_stacks: &mut Vec<VecDeque<char>>, lines: &mut Split<&str>) {
    loop {
        let line = lines.next().unwrap_or("");
        if line.len() == 0 {
            break;
        }

        let chars = line.chars().collect::<Vec<_>>();
        if chars.len() >= 2 && chars[0] == ' ' && chars[1] == '1' {
            let total_stacks = (line.trim_end().len() / 4) + 1;
            crate_stacks.resize_with(total_stacks, || VecDeque::new());
            break;
        }

        let chunks = chars.chunks(4);
        chunks.enumerate().for_each(|(i, chunk)| {
            if chunk.len() < 3 {
                return;
            }

            if chunk[0] == '[' {
                let c = chunk[1];
                if i >= crate_stacks.len() {
                    crate_stacks.resize_with(i + 1, || VecDeque::new());
                }
                crate_stacks[i].push_front(c);
            }
        });
    }
}
