use std::{cell::RefCell, rc::Rc};

/// Take as input a series of cd and ls commands.
/// Find the smallest directory to delete that frees up at least 30000000
/// in a filesystem with 70000000.
/// Sub directories and their parents are allowed to count twice.

#[derive(Debug)]
struct Dir {
    name: String,
    parent: Option<Rc<RefCell<Dir>>>,
    dirs: Vec<Rc<RefCell<Dir>>>,
    files: Vec<File>,
}

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

impl Dir {
    fn new(name: &str, parent: Option<Rc<RefCell<Dir>>>) -> Rc<RefCell<Dir>> {
        Rc::new(RefCell::new(Dir {
            name: String::from(name),
            parent,
            dirs: vec![],
            files: vec![],
        }))
    }

    fn add_dir(&mut self, name: &str, parent: Rc<RefCell<Dir>>) -> Rc<RefCell<Dir>> {
        let existing = self.dirs.iter().find(|dir| dir.borrow_mut().name == name);

        match existing {
            Some(dir) => {
                println!("SAME DIR");
                dir.clone()
            }
            None => {
                let new_dir = Dir::new(name, Option::from(parent));
                self.dirs.push(new_dir);

                self.dirs.last().unwrap().clone()
            }
        }
    }

    fn add_file(&mut self, file: File) {
        let existing = self.files.iter().find(|f| f.name == file.name);
        match existing {
            Some(_) => println!("SAME FILE"),
            None => self.files.push(file),
        }
    }

    fn size(&self) -> u64 {
        let file_sum: u64 = self.files.iter().map(|file| file.size).sum();

        let dir_sum: u64 = self.dirs.iter().map(|dir| dir.borrow_mut().size()).sum();

        file_sum + dir_sum
    }
}

fn create_tree(commands: Vec<Vec<&str>>) -> Rc<RefCell<Dir>> {
    let root = Dir::new("/", Option::None);
    let mut current = root.clone();

    for cmd in commands {
        match cmd[0] {
            "$" => {
                // This is a command
                match cmd[1] {
                    "cd" => {
                        if cmd[2] == "/" {
                            current = root.clone();
                        } else if cmd[2] == ".." {
                            let parent = current.borrow_mut().parent.as_ref().unwrap().clone();
                            current = parent;
                        } else {
                            let new_dir = current.borrow_mut().add_dir(cmd[2], current.clone());
                            current = new_dir;
                        }
                    }
                    "ls" => {
                        // Files will be listed do nothing
                    }
                    _ => panic!("Invalid command start"),
                }
            }
            "dir" => {
                // This is a dir listing from ls
                current.borrow_mut().add_dir(cmd[1], current.clone());
            }
            _ => {
                // This must be a file listing from ls
                current.borrow_mut().add_file(File {
                    name: String::from(cmd[1]),
                    size: cmd[0].parse::<u64>().unwrap(),
                })
            }
        }
    }

    root
}

fn find_smallest_to_free_size(root: Rc<RefCell<Dir>>) -> u64 {
    let unused = 70000000 - root.borrow().size();
    let needed = 30000000 - unused;

    let mut stack = vec![root];
    let mut best = u64::MAX;

    while !stack.is_empty() {
        let dir = stack.pop().unwrap();
        stack.append(&mut dir.borrow_mut().dirs.clone());

        if dir.borrow().size() >= needed && dir.borrow().size() < best {
            best = dir.borrow().size();
        }
    }

    best
}

fn print_tree(root: Rc<RefCell<Dir>>, level: usize) {
    print!("{:0width$}- ", "", width = level);
    print!("{}", root.borrow_mut().name);
    println!(" ({})", root.borrow_mut().size());
    for dir in &root.borrow_mut().dirs {
        print_tree(dir.clone(), level + 1);

        for file in &dir.borrow_mut().files {
            print!("{:0width$}+ ", "", width = level + 1);
            print!("{}", file.name);
            println!(" ({})", file.size);
        }
    }
}

fn main() {
    let input = include_str!("../../assets/day07.txt");

    let commands = input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let root = create_tree(commands);

    let size = find_smallest_to_free_size(root.clone());
    print_tree(root, 0);

    println!("Smallest to free size: {}", size);
}
