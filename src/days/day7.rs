use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    fs,
    rc::{Rc, Weak},
};

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct File {
    name: String,
    size: usize,
}
#[derive(Debug, Clone)]
pub struct Dir {
    name: String,
    // Child doesn't own parent.
    parent: RefCell<Weak<Dir>>,
    children: RefCell<Vec<Rc<Dir>>>,
    files: RefCell<Vec<File>>,
}

#[derive(Debug, Clone)]
pub struct FileSystem {
    dirs: Vec<Rc<Dir>>,
}

#[derive(Debug)]
struct FileSystemError {}

impl std::fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failure for Files.")
    }
}
impl Error for FileSystemError {}

impl Dir {
    fn du(&self) -> usize {
        let mut dir_size: usize = 0;
        for file in &*self.files.borrow() {
            dir_size += file.size
        }
        for dir in &*self.children.borrow() {
            dir_size += dir.du()
        }
        dir_size
    }
}

impl FileSystem {
    fn new(fname: &str) -> Result<FileSystem, Box<dyn Error>> {
        let contents = fs::read_to_string(fname)?;
        // Store directories and files.
        let mut dirs: Vec<Rc<Dir>> = vec![Rc::new(Dir {
            name: "/".to_string(),
            children: RefCell::new(vec![]),
            files: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        })];
        let curr_dir: RefCell<Option<Rc<Dir>>> = RefCell::new(Some(dirs.get(0).unwrap().clone()));

        // Iterate through the commmands and their outputs.
        for mut cmd_out in contents.trim().split("$") {
            cmd_out = cmd_out.trim();
            // If command is to change directory.
            if cmd_out.starts_with("cd") {
                if let Some(name) = cmd_out.split(" ").nth(1) {
                    if name == ".." {
                        let parent_dir = if let Some(c_dir) = &*curr_dir.borrow() {
                            c_dir.parent.borrow().upgrade()
                        } else {
                            None
                        };
                        *curr_dir.borrow_mut() = parent_dir;
                    } else if name == "/" {
                        // Skip root directory.
                        continue;
                    } else {
                        if let Ok(mut current_dir) = curr_dir.try_borrow_mut() {
                            if let Some(cdir) = &*current_dir {
                                let next_dir = cdir
                                    .children
                                    .borrow()
                                    .iter()
                                    .find(|dir| dir.name == name)
                                    .cloned();
                                *current_dir = next_dir
                            }
                        } else {
                            println!("Can't get current.")
                        }
                    }
                } else {
                    continue;
                };
            } else {
                // Otherwise is viewing directory structure.
                for mut desc_file in cmd_out.lines() {
                    desc_file = desc_file.trim();

                    if desc_file == "ls" {
                        continue;
                    }
                    let split_desc_file = desc_file.split(" ").collect_vec();
                    if let (Some(desc), Some(name)) =
                        (split_desc_file.get(0), split_desc_file.get(1))
                    {
                        // Add children to curr_dir ref
                        if *desc == "dir" {
                            if let Some(wd) = &*curr_dir.borrow_mut() {
                                // Get child directory in current
                                // Init with all fields empty.
                                let directory = Rc::new(Dir {
                                    name: name.to_string(),
                                    children: RefCell::new(vec![]),
                                    files: RefCell::new(vec![]),
                                    parent: RefCell::new(Rc::downgrade(&wd)),
                                });
                                dirs.push(directory.clone());
                                wd.children.borrow_mut().push(directory);
                            }
                        } else {
                            let file = File {
                                name: name.to_string(),
                                size: desc.parse::<usize>()?,
                            };
                            // Get a mut ref to curr directory.
                            if let Some(wd) = &*curr_dir.borrow_mut() {
                                wd.files.borrow_mut().push(file);
                            }
                        }
                    }
                }
            }
        }
        Ok(FileSystem { dirs })
    }
}

pub fn sum_file_system(fname: &str) -> Result<usize, Box<dyn Error>> {
    let file_system = FileSystem::new(fname)?;

    // Iterate through all dirs and calculate directory size.
    // du() calculates disk size for current directory and nested directories.
    let dir_sizes: HashMap<String, usize> = file_system
        .dirs
        .iter()
        .enumerate()
        .filter_map(|(i, dir)| {
            let dir_size = dir.du();
            let new_dirname = format!("{i}_{}", dir.name);
            if dir_size < 100000 {
                Some((new_dirname, dir_size))
            } else {
                None
            }
        })
        .collect();

    let mut total_disk_size: usize = 0;
    for (dir, size) in dir_sizes.iter() {
        println!("{:?} - {}", dir, size);
        total_disk_size += size;
    }

    Ok(total_disk_size)
}

pub fn free_space_file_system(fname: &str) -> Result<usize, Box<dyn Error>> {
    let file_system = FileSystem::new(fname)?;
    const DISK_SIZE: usize = 70_000_000;
    const REQ_DISK_SPACE: usize = 30_000_000;

    // Iterate through all dirs and calculate directory size.
    // du() calculates disk size for current directory and nested directories.
    let dir_sizes: Vec<(String, usize)> = file_system
        .dirs
        .iter()
        .enumerate()
        .filter_map(|(i, dir)| {
            let dir_size = dir.du();
            let new_dirname = format!("{i}_{}", dir.name);
            Some((new_dirname, dir_size))
        })
        .sorted_by(|(_, size_a), (_, size_b)| Ord::cmp(size_a, &size_b))
        .collect();

    // Final dir is root dir.
    let available_space = DISK_SIZE.saturating_sub(dir_sizes.last().unwrap().1);

    let mut del_dir_size: Option<usize> = None;
    for (_, size) in dir_sizes.iter() {
        let space_after_del = available_space + *size;
        if space_after_del > REQ_DISK_SPACE {
            // println!("{:?} - {} ({})", dir, size, space_after_del);
            del_dir_size = Some(*size);
            break;
        }
    }

    Ok(del_dir_size.unwrap())
}
