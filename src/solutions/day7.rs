use std::collections::HashMap;

use crate::Stage;

pub fn solve(stage: Stage, input: &Vec<String>) -> String {
    let mut tree = build_tree(input);
    tree.update_total_size();

    match stage {
        Stage::Easy => {solve_easy(&tree)}
        Stage::Hard => {solve_hard(&tree)}
    }
        .to_string()
}

fn solve_hard(tree: &Directory) -> usize {
    let required_size = 30000000 - (70000000 - tree.total_size);
    
    let mut size = usize::MAX;
    let mut visitor = |dir: &Directory| {
        if dir.total_size >= required_size && dir.total_size < size { size = dir.total_size }
    };

    tree.visit(&mut visitor);
    size
}

fn solve_easy(tree: &Directory) -> usize {
    let mut size = 0usize;
    let mut visitor = |dir: &Directory| {
        if dir.total_size <= 100000 { size += dir.total_size }
    };

    tree.visit(&mut visitor);
    size
}

fn build_tree(input: &Vec<String>) -> Directory {
    let mut root = Directory::new();
    root.add_dir("/");
    let mut cwd = &mut root;

    let mut stack: Vec<&str> = Vec::new();

    for row in input {
        if row == "$ ls" {
            continue;
        }

        let target_dir = row.strip_prefix("$ cd ");
        if let Some(target_dir) = target_dir {
            match target_dir {
                ".." => {
                    stack.pop();
                }
                child => {
                    stack.push(child);
                }
            }
            cwd = root.get_child_dir_mut(&stack).unwrap();

            continue;
        }

        let [sz_or_dir, name] = row.split(' ').collect::<Vec<&str>>()[..] else {
            unreachable!()
        };
        match sz_or_dir {
            "dir" => {
                cwd.add_dir(name);
            }
            size => {
                cwd.add_file(name, size.parse().unwrap());
            }
        }
    }

    root
}

struct Directory {
    total_size: usize,
    files: HashMap<String, usize>,
    sub_dirs: HashMap<String, Directory>,
}

impl Directory {
    fn new() -> Directory {
        Directory {
            total_size: 0,
            files: HashMap::new(),
            sub_dirs: HashMap::new(),
        }
    }

    fn update_total_size(&mut self) {
        for (_, sub_dir) in self.sub_dirs.iter_mut() {
            sub_dir.update_total_size();
        }

        self.total_size = self
            .sub_dirs
            .iter()
            .map(|(_, d)| d.total_size)
            .sum::<usize>()
            + self.files.iter().map(|(_, s)| s).sum::<usize>();
    }

    fn add_file(&mut self, name: &str, size: usize) {
        self.files.insert(name.to_string(), size);
    }

    fn add_dir(&mut self, name: &str) -> &Directory {
        let dir = Directory::new();
        self.sub_dirs.insert(name.to_string(), dir);
        &self.sub_dirs[name]
    }

    fn get_child_dir_mut(&mut self, path: &[&str]) -> Option<&mut Directory> {
        let mut result = self;

        for &dir in path {
            result = result.sub_dirs.get_mut(dir)?;
        }

        Some(result)
    }
    
    fn visit(&self, cb: &mut impl FnMut(&Self) -> ()) {
        cb(self);
        for value in self.sub_dirs.values() {
            value.visit(cb);
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        print_impl(self, 0);

        fn print_impl(root: &Directory, level: usize) {
            let (padding_normal, padding_last) = get_paddings(level);

            for (name, dir) in &root.sub_dirs {
                println!("{}{}\t\t({})", padding_normal, name, dir.total_size);
                print_impl(dir, level + 1);
            }

            for (i, (name, size)) in root.files.iter().enumerate() {
                let padding = if i + 1 == root.files.len() {
                    &padding_last
                } else {
                    &padding_normal
                };
                println!("{}{}\t\t({})", padding, name, size);
            }

            fn get_paddings(level: usize) -> (String, String) {
                if level == 0 {
                    return ("".to_string(), "".to_string());
                }

                let mut base = String::new();
                
                for _ in 0..level - 1 {
                    base.push('│');
                }

                (format!("{}├", base), format!("{}└", base))
            }
        }
    }
}
