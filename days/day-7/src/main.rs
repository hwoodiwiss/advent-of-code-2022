use std::{
    cell::RefCell,
    io::{self, BufRead},
    rc::Rc,
};

trait Sized {
    fn size(&self) -> usize;
}

struct File {
    pub name: String,
    pub size: usize,
}

impl File {
    fn new(name: &str, size: &str) -> Self {
        Self {
            name: name.to_owned(),
            size: size.parse().unwrap(),
        }
    }
}

impl Sized for File {
    fn size(&self) -> usize {
        self.size
    }
}

struct Dir {
    parent: Option<Rc<RefCell<Dir>>>,
    name: String,
    items: Vec<DirectoryNode>,
}

impl Dir {
    fn new(name: &str, parent: Rc<RefCell<Dir>>) -> Self {
        Self {
            parent: Some(parent),
            name: name.to_owned(),
            items: Vec::new(),
        }
    }

    fn new_root(name: &str) -> Self {
        Self {
            parent: None,
            name: name.to_owned(),
            items: Vec::new(),
        }
    }

    fn add_node(&mut self, node: DirectoryNode) {
        self.items.push(node);
    }

    fn find_dir(&self, path: &str) -> Option<Rc<RefCell<Dir>>> {
        for node in &self.items {
            if let DirectoryNode::Dir(dir) = node {
                if dir.borrow().name == path.to_owned() {
                    return Some(dir.clone());
                }
            }
        }

        None
    }
}

impl Sized for Dir {
    fn size(&self) -> usize {
        self.items.iter().map(|n| n.size()).sum()
    }
}
enum DirectoryNode {
    File(Rc<RefCell<File>>),
    Dir(Rc<RefCell<Dir>>),
}

impl Sized for DirectoryNode {
    fn size(&self) -> usize {
        match self {
            DirectoryNode::File(file) => file.borrow().size(),
            DirectoryNode::Dir(dir) => dir.borrow().size(),
        }
    }
}

enum Command {
    Cd(String),
    CdUp,
    Ls,
}

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().map(|ln| ln.unwrap()).collect();
    let directory = create_dir_structure(&lines);
    println!("Part 1: {}", get_sum_dirs_smaller_than(&directory, 100000));

    let space_to_clear = 30000000 - (70000000 - directory.size());
    let mut dirs_bigger_than_threshold = Vec::new();
    get_all_dirs_greater_than(&directory, space_to_clear, &mut dirs_bigger_than_threshold);

    dirs_bigger_than_threshold.sort();

    println!("Part 2: {}", dirs_bigger_than_threshold.first().unwrap());
}

fn create_dir_structure(commands: &Vec<String>) -> DirectoryNode {
    let root_node = DirectoryNode::Dir(Rc::new(RefCell::new(Dir::new_root("/"))));
    let mut curr_dir = match &root_node {
        DirectoryNode::Dir(dir) => dir.clone(),
        _ => unreachable!(),
    };

    for line in &commands[..] {
        if let Some(node) = try_parse_dir_node(&line, curr_dir.clone()) {
            curr_dir.borrow_mut().add_node(node);
            continue;
        }
        if let Some(command) = try_parse_command(&line) {
            match command {
                Command::Cd(dir) => {
                    if let Some(dir) = &curr_dir.clone().borrow().find_dir(&dir) {
                        curr_dir = dir.clone();
                    }
                }
                Command::CdUp => {
                    if let Some(dir) = &curr_dir.clone().borrow().parent {
                        curr_dir = dir.clone();
                    };
                }
                Command::Ls => continue,
            }
            continue;
        }
    }

    root_node
}

fn try_parse_command(command_line: &str) -> Option<Command> {
    let parts = command_line.split(' ').collect::<Vec<_>>();

    match parts[1] {
        "cd" if parts[2] == ".." => Some(Command::CdUp),
        "cd" => Some(Command::Cd(parts[2].to_owned())),
        "ls" => Some(Command::Ls),
        _ => None,
    }
}

fn try_parse_dir_node(node_text: &str, parent_node: Rc<RefCell<Dir>>) -> Option<DirectoryNode> {
    let parts = node_text.split(' ').collect::<Vec<_>>();

    match parts[0] {
        "$" => None,
        "dir" => Some(DirectoryNode::Dir(Rc::new(RefCell::new(Dir::new(
            parts[1],
            parent_node,
        ))))),
        _ => Some(DirectoryNode::File(Rc::new(RefCell::new(File::new(
            parts[1], parts[0],
        ))))),
    }
}

fn get_sum_dirs_smaller_than(dir_node: &DirectoryNode, max_size: usize) -> usize {
    if let DirectoryNode::Dir(dir) = dir_node {
        let dir = dir.clone();
        let size = dir.borrow().size();
        let curr_size = if dir.borrow().size() <= max_size {
            size
        } else {
            0usize
        };
        curr_size
            + dir
                .clone()
                .borrow()
                .items
                .iter()
                .map(|m| get_sum_dirs_smaller_than(m, max_size))
                .sum::<usize>()
    } else {
        0usize
    }
}

fn get_all_dirs_greater_than(dir_node: &DirectoryNode, min_size: usize, dirs: &mut Vec<usize>) {
    if let DirectoryNode::Dir(dir) = dir_node {
        let dir = dir.clone();
        if dir.borrow().size() >= min_size {
            dirs.push(dir.borrow().size());
        }
        dir.clone()
            .borrow()
            .items
            .iter()
            .for_each(|m| get_all_dirs_greater_than(&m, min_size, dirs))
    };
}

#[cfg(test)]
mod test {
    use crate::{
        create_dir_structure, get_all_dirs_greater_than, get_sum_dirs_smaller_than, DirectoryNode,
        Sized,
    };

    #[test]
    fn test_create_dir_structure() {
        let commands = Vec::from([
            "$ cd /".to_owned(),
            "$ ls".to_owned(),
            "dir a".to_owned(),
            "14848514 b.txt".to_owned(),
            "8504156 c.dat".to_owned(),
            "dir d".to_owned(),
            "$ cd a".to_owned(),
            "$ ls".to_owned(),
            "dir e".to_owned(),
            "29116 f".to_owned(),
            "2557 g".to_owned(),
            "62596 h.lst".to_owned(),
            "$ cd e".to_owned(),
            "$ ls".to_owned(),
            "584 i".to_owned(),
            "$ cd ..".to_owned(),
            "$ cd ..".to_owned(),
            "$ cd d".to_owned(),
            "$ ls".to_owned(),
            "4060174 j".to_owned(),
            "8033020 d.log".to_owned(),
            "5626152 d.ext".to_owned(),
            "7214296 k".to_owned(),
        ]);

        let actual_directory_node = create_dir_structure(&commands);
        let actual_root_directory = if let DirectoryNode::Dir(dir) = actual_directory_node {
            dir.clone()
        } else {
            panic!("")
        };

        let actual_a_dir = actual_root_directory.borrow().find_dir("a").unwrap();

        assert_eq!(actual_a_dir.borrow().size(), 94853);

        let actual_e_dir = actual_a_dir.borrow().find_dir("e").unwrap();

        assert_eq!(actual_e_dir.borrow().size(), 584);

        let actual_d_dir = actual_root_directory.borrow().find_dir("d").unwrap();

        assert_eq!(actual_d_dir.borrow().size(), 24933642);

        assert_eq!(actual_root_directory.borrow().size(), 48381165);
    }

    #[test]
    fn test_get_directory_nodes_smaller_than() {
        let commands = Vec::from([
            "$ cd /".to_owned(),
            "$ ls".to_owned(),
            "dir a".to_owned(),
            "14848514 b.txt".to_owned(),
            "8504156 c.dat".to_owned(),
            "dir d".to_owned(),
            "$ cd a".to_owned(),
            "$ ls".to_owned(),
            "dir e".to_owned(),
            "29116 f".to_owned(),
            "2557 g".to_owned(),
            "62596 h.lst".to_owned(),
            "$ cd e".to_owned(),
            "$ ls".to_owned(),
            "584 i".to_owned(),
            "$ cd ..".to_owned(),
            "$ cd ..".to_owned(),
            "$ cd d".to_owned(),
            "$ ls".to_owned(),
            "4060174 j".to_owned(),
            "8033020 d.log".to_owned(),
            "5626152 d.ext".to_owned(),
            "7214296 k".to_owned(),
        ]);

        let directory_node = create_dir_structure(&commands);
        let sum_dirs_lt_100k = get_sum_dirs_smaller_than(&directory_node, 100000);

        assert_eq!(sum_dirs_lt_100k, 95437)
    }

    #[test]
    fn test_get_all_dirs_greater_than() {
        let commands = Vec::from([
            "$ cd /".to_owned(),
            "$ ls".to_owned(),
            "dir a".to_owned(),
            "14848514 b.txt".to_owned(),
            "8504156 c.dat".to_owned(),
            "dir d".to_owned(),
            "$ cd a".to_owned(),
            "$ ls".to_owned(),
            "dir e".to_owned(),
            "29116 f".to_owned(),
            "2557 g".to_owned(),
            "62596 h.lst".to_owned(),
            "$ cd e".to_owned(),
            "$ ls".to_owned(),
            "584 i".to_owned(),
            "$ cd ..".to_owned(),
            "$ cd ..".to_owned(),
            "$ cd d".to_owned(),
            "$ ls".to_owned(),
            "4060174 j".to_owned(),
            "8033020 d.log".to_owned(),
            "5626152 d.ext".to_owned(),
            "7214296 k".to_owned(),
        ]);

        let directory_node = create_dir_structure(&commands);
        let curr_used = directory_node.size();
        let space_to_clear = 30000000 - (70000000 - curr_used);
        let mut dirs_gt_8381165 = Vec::new();
        get_all_dirs_greater_than(&directory_node, space_to_clear, &mut dirs_gt_8381165);

        dirs_gt_8381165.sort();

        assert_eq!(*dirs_gt_8381165.first().unwrap(), 24933642);
    }
}
