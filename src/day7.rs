use std::{iter::repeat, str::Split};

use crate::utils::must_read_file;

pub fn part1() {
    let data = must_read_file("data/day7/part1.txt");
    let lines = data.trim().split("\n");

    let mut fs = FileSystem::new();
    fs.build(lines);
    fs.print();

    println!(
        "Total sum: {}",
        fs.dirs
            .iter()
            .map(|d| d.size)
            .filter(|s| *s <= 100000)
            .sum::<u64>()
    )
}

pub fn part2() {
    let data = must_read_file("data/day7/part2.txt");
    let lines = data.trim().split("\n");

    let mut fs = FileSystem::new();
    fs.build(lines);
    fs.print();

    let max_allowed_size = 40000000;
    let total_size = fs.dirs[0].size;
    if total_size <= max_allowed_size {
        println!("Nothing to delete");
        return;
    }

    let to_delete = total_size - max_allowed_size;

    println!(
        "Directory size to delete: {}",
        fs.dirs
            .iter()
            .map(|d| d.size)
            .filter(|s| *s >= to_delete)
            .min()
            .unwrap_or(0)
    )
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    LS,
    CD,
    None,
}

#[derive(Debug)]
struct File {
    index: usize,
    name: String,
    size: u64,
}

#[derive(Debug)]
struct Dir {
    index: usize,
    name: String,
    size: u64,
    dirs: Vec<usize>,
    files: Vec<usize>,
    parent: Option<usize>,
}

#[derive(Debug)]
struct FileSystem {
    dirs: Vec<Dir>,
    files: Vec<File>,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            dirs: vec![Dir {
                index: 0,
                name: "/".to_string(),
                size: 0,
                dirs: Vec::new(),
                files: Vec::new(),
                parent: None,
            }],
            files: Vec::new(),
        }
    }

    fn build(&mut self, lines: Split<&str>) {
        let mut curr_index = 0usize;

        let mut state = State::None;
        for line in lines {
            let line = line.trim();
            if line.starts_with('$') {
                let cmd_line = line[1..].trim();
                if cmd_line == "ls" {
                    state = State::LS;
                } else if cmd_line.starts_with("cd ") {
                    let dir_name = cmd_line[3..].trim();
                    if dir_name == "/" {
                        curr_index = 0;
                    } else if dir_name == ".." {
                        if let Some(p) = self.dirs[curr_index].parent {
                            curr_index = p;
                        }
                    } else {
                        if let Some(dir_index) = self.find_dir(curr_index, dir_name) {
                            curr_index = dir_index;
                        } else {
                            curr_index = self.add_dir(curr_index, dir_name);
                        }
                    }
                    state = State::CD;
                } else {
                    state = State::None;
                }
            } else if state == State::LS {
                if line.starts_with("dir ") {
                    let dir_name = line[4..].trim();
                    if let None = self.find_dir(curr_index, dir_name) {
                        self.add_dir(curr_index, dir_name);
                    }
                } else {
                    let parts = line.split(" ").map(|x| x.trim()).collect::<Vec<_>>();
                    if parts.len() == 2 {
                        if let Ok(size) = parts[0].parse::<u64>() {
                            let file_name = parts[1];
                            if let None = self.find_file(curr_index, file_name) {
                                self.add_file(curr_index, file_name, size);
                            }
                        }
                    }
                }
            }
        }

        self.update_sizes();
    }

    fn find_dir(&self, curr_index: usize, name: &str) -> Option<usize> {
        for dir_index in self.dirs[curr_index].dirs.iter() {
            let dir = &self.dirs[*dir_index];
            if dir.name == name {
                return Some(dir.index);
            }
        }
        None
    }

    fn add_dir(&mut self, curr_index: usize, name: &str) -> usize {
        let index = self.dirs.len();
        let new_dir = Dir {
            index,
            name: name.to_string(),
            size: 0,
            dirs: Vec::new(),
            files: Vec::new(),
            parent: Some(curr_index),
        };
        self.dirs.push(new_dir);
        self.dirs[curr_index].dirs.push(index);
        index
    }

    fn find_file(&self, curr_index: usize, name: &str) -> Option<usize> {
        for file_index in self.dirs[curr_index].files.iter() {
            let file = &self.files[*file_index];
            if file.name == name {
                return Some(file.index);
            }
        }
        None
    }

    fn add_file(&mut self, curr_index: usize, name: &str, size: u64) -> usize {
        let index = self.files.len();
        let new_dir = File {
            index,
            name: name.to_string(),
            size,
        };
        self.files.push(new_dir);
        self.dirs[curr_index].files.push(index);
        index
    }

    fn update_sizes(&mut self) {
        self.update_sizes_dir(0);
    }

    fn update_sizes_dir(&mut self, index: usize) {
        let dirs = self.dirs[index].dirs.clone();
        for sub_dir_index in dirs.iter() {
            self.update_sizes_dir(*sub_dir_index);
            self.dirs[index].size += self.dirs[*sub_dir_index].size;
        }

        let files = self.dirs[index].files.clone();
        for sub_file_index in files.iter() {
            self.dirs[index].size += self.files[*sub_file_index].size;
        }
    }

    fn print(&self) {
        println!();
        self.print_dir(0, 0);
        println!();
    }

    fn print_dir(&self, gap: usize, index: usize) {
        let dir = &self.dirs[index];
        println!(
            "{}- {} (dir, size={})",
            repeat(" ").take(gap * 2).collect::<String>(),
            dir.name,
            dir.size,
        );
        for sub_dir_index in dir.dirs.iter() {
            self.print_dir(gap + 1, *sub_dir_index)
        }
        for sub_file_index in dir.files.iter() {
            self.print_file(gap + 1, *sub_file_index)
        }
    }

    fn print_file(&self, gap: usize, index: usize) {
        let file = &self.files[index];
        println!(
            "{}- {} (file, size={})",
            repeat(" ").take(gap * 2).collect::<String>(),
            file.name,
            file.size,
        );
    }
}
