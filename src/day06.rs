use std::collections::VecDeque;

use crate::utils::must_read_file;

pub fn part1() {
    let data = must_read_file("data/day06/part1.txt");
    let chars = data.trim().chars().collect::<Vec<_>>();
    if chars.len() < 4 {
        println!("Not enough data");
    }

    let marker_index = get_marker_index(&chars, 4);
    println!("Marker index: {}", marker_index + 1);
}

pub fn part2() {
    let data = must_read_file("data/day06/part2.txt");
    let chars = data.trim().chars().collect::<Vec<_>>();
    if chars.len() < 14 {
        println!("Not enough data");
    }

    let marker_index = get_marker_index(&chars, 14);
    println!("Marker index: {}", marker_index + 1);
}

fn get_marker_index(chars: &Vec<char>, len: usize) -> usize {
    let mut curr_window = VecDeque::from(chars[0..len].to_vec());
    if is_window_unique(&curr_window, len) {
        return 3;
    }
    for i in len..chars.len() {
        let curr_char = chars[i];
        curr_window.pop_front();
        curr_window.push_back(curr_char);
        if is_window_unique(&curr_window, len) {
            return i;
        }
    }
    chars.len()
}

fn is_window_unique(window: &VecDeque<char>, len: usize) -> bool {
    if window.len() < len {
        return false;
    }

    for i in 0..window.len() - 1 {
        for j in i + 1..window.len() {
            if window[i] == window[j] {
                return false;
            }
        }
    }
    return true;
}
