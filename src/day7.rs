use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use scan_fmt::scan_fmt;

enum Row  {
    Cd(String),
    Ls,
    DirEntry(String),
    FileEntry(usize, String)
}

fn parse_row(row: &str) -> Row {
    if let Ok(folder) = scan_fmt!(row, "$ cd {}", String) {
        return Row::Cd(folder)
    }

    if row == "$ ls" {
        return Row::Ls
    }

    if let Ok(folder) = scan_fmt!(row, "dir {}", String) {
        return Row::DirEntry(folder)
    }

    if let Ok((size, file)) = scan_fmt!(row, "{d} {}", usize, String) {
        return Row::FileEntry(size, file)
    }

    println!("{}", row);
    panic!("Unknown command");
}

enum FSEntry {
    Directory,
    File(usize)
}

pub fn run_day7() {
    println!("Starting day 7!");

    let mut f = File::open("data/day7.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let commands = s.split("\n").map(
        |row| parse_row(row)
    ).collect::<Vec<_>>();

    let mut cwd = vec!();
    let mut entries: HashMap<String, FSEntry> = HashMap::new();

    for command in commands.iter() {
        match &*command {
            Row::Ls => { },
            Row::Cd(directory) => {
                if directory == "/" {
                    cwd.clear();
                } else if directory == ".." {
                    cwd.pop();
                } else {
                    cwd.push(directory.as_str());
                }
            },
            Row::DirEntry(name) => {
                entries.insert(cwd.join("/") + "/" + name, FSEntry::Directory);
            },
            Row::FileEntry(size, name) => {
                entries.insert(cwd.join("/") + "/" + name, FSEntry::File(*size));
            }
        }
    }

    let mut directory_sizes: HashMap<String, usize> = HashMap::new();

    for (path, entry) in entries.iter() {
        if let FSEntry::Directory = entry {
            let mut total_size = 0;

            for (subpath, entry) in entries.iter() {
                if subpath.starts_with(path) {
                    if let FSEntry::File(file_size) = entry {
                        total_size += file_size;
                    }
                }
            }

            directory_sizes.insert(path.clone(), total_size);
        }
    }

    let total_filtered: usize = directory_sizes.iter().map(|(_, s)| *s).filter(|s| *s <= 100000).sum();

    println!("Part 1 total: {}", total_filtered);

    let total_used: usize = 
        entries
            .iter()
            .map(|(_, f)| if let FSEntry::File(size) = f { *size } else {0})
            .sum();

    let free_space = 70000000 - total_used;
    let required_space = 30000000 - free_space;

    println!("Required space: {}", required_space);

    let mut directories = 
        directory_sizes.iter().filter(|(_, s)| **s > required_space)
        .collect::<Vec<_>>();

    directories.sort_by_key(|(_, s)| *s);

    println!("Directory size to delete: {}", directories.first().expect("").1);
}