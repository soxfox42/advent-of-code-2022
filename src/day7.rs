use std::collections::HashMap;

#[derive(Debug)]
enum Item {
    File(usize),
    Directory(HashMap<String, Item>, usize),
}
impl Item {
    fn size(&self) -> usize {
        match self {
            &Self::File(size) => size,
            &Self::Directory(_, size) => size,
        }
    }

    fn update_directory_sizes(&mut self) {
        match self {
            Self::File(_) => {
                return;
            }
            Self::Directory(map, size) => {
                for item in map.values_mut() {
                    item.update_directory_sizes();
                }
                *size = map.values().map(Self::size).sum();
            }
        }
    }

    fn small_dir_sum(&self) -> usize {
        match self {
            Self::File(_) => 0,
            Self::Directory(items, size) => {
                if *size <= 100000 {
                    size + items.values().map(Self::small_dir_sum).sum::<usize>()
                } else {
                    items.values().map(Self::small_dir_sum).sum()
                }
            }
        }
    }
}

fn get_path<'a>(tree: &'a mut Item, path: &[String]) -> &'a mut Item {
    let mut cwd = tree;

    for component in path {
        let Item::Directory(map, _) = cwd else { panic!() };
        cwd = map.get_mut(component).unwrap();
    }

    cwd
}

fn read_tree(input: &str) -> Item {
    let mut root = Item::Directory(HashMap::new(), 0);
    let mut path = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ cd") {
            let dir = line.split_whitespace().nth(2).unwrap();
            match dir {
                "/" => (),
                ".." => {
                    path.pop();
                }
                dir => path.push(dir.to_string()),
            }
        } else if line.starts_with("dir") {
            let name = line.split_whitespace().nth(1).unwrap().to_string();
            let Item::Directory(map, _) = get_path(&mut root, &path) else { panic!() };
            map.insert(name, Item::Directory(HashMap::new(), 0));
        } else {
            let mut tokens = line.split_whitespace();
            let Ok(size) = tokens.next().unwrap().parse() else { continue };
            let name = tokens.next().unwrap().to_string();
            let Item::Directory(map, _) = get_path(&mut root, &path) else { panic!() };
            map.insert(name, Item::File(size));
        }
    }

    root.update_directory_sizes();
    root
}

pub fn run_a(input: &str) {
    println!(
        "Sum of small directory sizes: {}",
        read_tree(input).small_dir_sum()
    );
}

pub fn run_b(_input: &str) {}
