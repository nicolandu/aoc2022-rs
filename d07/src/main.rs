use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io;

const THRESHOLD: u32 = 100_000;
const MAX_SIZE: u32 = 40_000_000;

/// Returns the root directory of the tree.
fn parse_commands(commands: &str) -> HashMap<String, u32> {
    let cd_parent = Regex::new(r"^\$ cd \.\.").unwrap();
    let cd_root = Regex::new(r"^\$ cd /").unwrap();
    let cd_child = Regex::new(r"^\$ cd (?P<name>[[:alpha:]]+)$").unwrap();

    // Good enough for our purposes
    let file = Regex::new(r"^(?P<size>([1-9][0-9]*|[0-9])) (?P<name>[!-.0-~]*)$").unwrap();

    let nop = Regex::new(r"^(\$ ls|dir [[:alpha:]]+)$").unwrap();

    let mut map = HashMap::new();

    // Structure: ./a/b/c/file.txt
    let mut cwd = String::new();

    // We assume we only visit each folder once, so we cut corners a bit.
    for command in commands.lines() {
        if cd_parent.is_match(command) {
            let i = cwd
                .rfind('/')
                .unwrap_or_else(|| panic!("Couldn't rfind a /: \"{cwd}\""));
            cwd.truncate(i);
        } else if cd_root.is_match(command) {
            cwd = ".".to_string();
        } else if let Some(child) = cd_child.captures(command) {
            cwd.push('/');
            cwd.push_str(&child["name"]);
        } else if let Some(file) = file.captures(command) {
            let file_sz = file["size"].parse::<u32>().unwrap();
            let dir_sz = map.entry(cwd.clone()).or_insert(0);
            *dir_sz += file_sz;

            // Increment parent directories
            cwd.match_indices('/').for_each(|(i, _)| {
                let dir_sz = map.entry(cwd[0..i].to_owned()).or_insert(0);
                *dir_sz += file_sz;
            });
        } else if !nop.is_match(command) {
            panic!("Invalid instruction: {}", command);
        }
    }

    map
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = io::read_to_string(io::stdin())?;
    let root = parse_commands(&content);

    let space_to_free = root["."] - MAX_SIZE;

    let total = root.values().filter(|&&s| s <= THRESHOLD).sum::<u32>();
    let smallest = root
        .values()
        .filter(|&&s| s > space_to_free)
        .min()
        .expect("Couldn't find smallest");
    println!("Total size of <= {THRESHOLD}: {total}, Best to delete: {smallest}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            parse_commands(
                "\
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
"
            ),
            [
                (".".to_string(), 48381165),
                ("./a".to_string(), 94853),
                ("./a/e".to_string(), 584),
                ("./d".to_string(), 24933642),
            ]
            .into()
        );
    }
}
