use std::{cell::RefCell, fs, path::PathBuf, rc::Rc};

#[derive(Debug)]
enum ChangeDirectoryParameter {
    Previous,
    Next(String),
}

#[derive(Debug)]
enum Command {
    ChangeDirectory(ChangeDirectoryParameter),
    List,
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Directory(String),
    File(String, usize),
}

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("file must be readable");

    let lines = input
        .lines()
        .skip(1)
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| parse_line(l));

    let mut file_system = FileSystem::new();

    for line in lines {
        match line {
            Line::Command(Command::ChangeDirectory(p)) => file_system.change_directory(p),
            Line::Command(Command::List) => (),
            Line::Directory(name) => {
                let new_dir = Directory::new(name, Rc::clone(&file_system.current_directory));
                file_system.add_directory_to_current(new_dir)
            }
            Line::File(name, size) => file_system.add_file_to_current(File::new(name, size)),
        }
    }

    let total = file_system.get_root().borrow().get_size();

    println!("{total:?}");

    (String::new(), String::new())
}

fn parse_line(line: &str) -> Option<Line> {
    if line == "$ ls" {
        return Some(Line::Command(Command::List));
    }

    let parts: Vec<&str> = line.split(" ").collect();

    if parts[0] == "$" {
        let param = match parts[2] {
            ".." => ChangeDirectoryParameter::Previous,
            s => ChangeDirectoryParameter::Next(s.to_string()),
        };

        return Some(Line::Command(Command::ChangeDirectory(param)));
    }

    if parts[0] == "dir" {
        return Some(Line::Directory(parts[1].to_string()));
    }

    parts[0]
        .parse::<usize>()
        .and_then(|s| Ok(Line::File(parts[1].to_string(), s)))
        .ok()
}

#[derive(Debug)]
struct FileSystem {
    root: Rc<RefCell<Directory>>,
    current_directory: Rc<RefCell<Directory>>,
}

impl FileSystem {
    fn new() -> FileSystem {
        let root = Rc::new(RefCell::new(Directory::new_root()));

        FileSystem {
            root: Rc::clone(&root),
            current_directory: Rc::clone(&root),
        }
    }

    fn change_directory(&mut self, param: ChangeDirectoryParameter) {
        self.current_directory = match param {
            ChangeDirectoryParameter::Previous => {
                let previous = self
                    .current_directory
                    .borrow()
                    .get_parent()
                    .expect("arrived at the root");
                Rc::clone(&previous)
            }
            ChangeDirectoryParameter::Next(name) => {
                let current = self.current_directory.borrow();
                let dir = current
                    .directories
                    .iter()
                    .find(|d| d.borrow().name == name)
                    .expect("directory does not exist");

                Rc::clone(&dir)
            }
        }
    }

    fn add_directory_to_current(&mut self, directory: Directory) {
        (*self.current_directory)
            .borrow_mut()
            .add_directory(Rc::new(RefCell::new(directory)))
    }

    fn add_file_to_current(&mut self, file: File) {
        (*self.current_directory).borrow_mut().add_file(file)
    }

    fn get_root(&self) -> Rc<RefCell<Directory>> {
        Rc::clone(&self.root)
    }
}

trait Size {
    fn get_size(&self) -> usize;
}

trait InDirectory {
    fn get_parent(&self) -> Option<Rc<RefCell<Directory>>>;
}

trait HoldsFiles {
    fn add_file(&mut self, file: File);
}

trait HoldsDirectories {
    fn add_directory(&mut self, directory: Rc<RefCell<Directory>>);
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: String, size: usize) -> Self {
        File { name, size }
    }
}

impl Size for File {
    fn get_size(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<Rc<RefCell<Directory>>>,
    parent: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    fn new(name: String, parent: Rc<RefCell<Directory>>) -> Self {
        Directory {
            name,
            files: Vec::new(),
            directories: Vec::new(),
            parent: Some(Rc::clone(&parent)),
        }
    }

    fn new_root() -> Directory {
        Directory {
            name: String::from("/"),
            files: Vec::new(),
            directories: Vec::new(),
            parent: None,
        }
    }
}

impl HoldsFiles for Directory {
    fn add_file(&mut self, file: File) {
        self.files.push(file)
    }
}

impl HoldsDirectories for Directory {
    fn add_directory(&mut self, directory: Rc<RefCell<Directory>>) {
        self.directories.push(Rc::clone(&directory))
    }
}

impl InDirectory for Directory {
    fn get_parent(&self) -> Option<Rc<RefCell<Directory>>> {
        match &self.parent {
            Some(p) => Some(Rc::clone(&p)),
            None => None,
        }
    }
}

impl Size for Directory {
    fn get_size(&self) -> usize {
        let files_size: usize = self.files.iter().map(|f| f.get_size()).sum();
        let dir_size: usize = self
            .directories
            .iter()
            .map(|d| (*d).borrow().get_size())
            .sum();

        files_size + dir_size
    }
}
