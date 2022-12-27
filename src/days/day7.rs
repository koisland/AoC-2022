use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Ref, RefCell},
    error::Error,
    fs,
    rc::{Rc, Weak}, collections::HashMap,
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
                        // Fails due to non-unique filename.
                        let next_dir = dirs.iter().find(|dir| dir.name == name).cloned();
                        *curr_dir.borrow_mut() = next_dir
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
    println!("{:#?}", file_system);
    // Iterate through all dirs and calculate directory size.
    // du() calculates disk size for current directory and nested directories.
    let dir_sizes: HashMap<String, usize> = file_system.dirs
        .iter()
        .filter_map(|dir| {
            let dir_size = dir.du();
            if dir_size < 100000 {
                Some((dir.name.to_string(), dir_size))
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
    println!("{}", total_disk_size);
    Ok(0)
}
