use crate::utils::must_read_file;
use std::collections::BTreeSet;
use std::ops::BitAnd;

pub fn part1() {
    let data = must_read_file("data/day03/part1.txt");
    let lines = data.trim().split("\n");
    let scores = lines.map(|line| {
        let len = line.len();
        let mid = len / 2;
        let half1 = &line[0..mid];
        let half2 = &line[mid..];
        find_common_char(&[half1, half2])
            .map(get_char_value)
            .unwrap_or(0)
    });
    let total_score = scores.sum::<u64>();
    println!("Total score: {total_score}");
}

pub fn part2() {
    let data = must_read_file("data/day03/part2.txt");
    let lines = data.trim().split("\n").collect::<Vec<_>>();
    let chunks = lines.chunks(3);
    let scores = chunks.map(|chunk| find_common_char(chunk).map(get_char_value).unwrap_or(0));
    let total_score = scores.sum::<u64>();
    println!("Total score: {total_score}");
}

fn find_common_char(strings: &[&str]) -> Option<char> {
    let len = strings.len();
    if len == 0 {
        None
    } else if len == 1 {
        strings[0].chars().next()
    } else {
        let first_set = strings[0].chars().collect::<BTreeSet<_>>();
        let sets = strings[1..]
            .iter()
            .map(|s| s.chars().collect::<BTreeSet<_>>())
            .collect::<Vec<_>>();
        let iter = sets.into_iter();
        iter.fold(first_set, |set1, set2| set1.bitand(&set2))
            .iter()
            .next()
            .copied()
    }
}

fn get_char_value(c: char) -> u64 {
    if c >= 'a' && c <= 'z' {
        (c as u64) - ('a' as u64) + 1
    } else if c >= 'A' && c <= 'Z' {
        (c as u64) - ('A' as u64) + 27
    } else {
        0
    }
}
