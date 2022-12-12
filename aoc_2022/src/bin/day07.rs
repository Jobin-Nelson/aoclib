use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Dir {
    _name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Dir>>,
    subdir: RefCell<HashMap<String, Rc<Dir>>>,
}

impl Dir {
    fn get_size(&self) -> usize {
        *self.size.borrow()
            + self
                .subdir
                .borrow()
                .values()
                .fold(0, |acc, v| acc + v.get_size())
    }
}

fn main() {
    let input = std::fs::read_to_string("data/day07.txt").unwrap();

    let mut cwd = Rc::new(Dir {
        _name: String::new(),
        size: RefCell::new(0),
        parent: None,
        subdir: RefCell::new(HashMap::new()),
    });

    let mut root = Rc::clone(&cwd);

    for line in input.lines() {
        let words = line.split(' ').collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "ls") => {}
            ("$", "cd") => match words[2] {
                "/" => cwd = Rc::clone(&root),
                ".." => cwd = Rc::clone(cwd.parent.as_ref().unwrap()),
                dirname => {
                    let new_dir = cwd.subdir.borrow().get(dirname).unwrap().clone();
                    cwd = new_dir;
                }
            },
            ("dir", dirname) => {
                cwd.subdir.borrow_mut().insert(
                    dirname.to_string(),
                    Rc::new(Dir {
                        _name: dirname.to_string(),
                        size: RefCell::new(0),
                        parent: Some(Rc::clone(&cwd)),
                        subdir: RefCell::new(HashMap::new()),
                    }),
                );
            }
            (size, _name) => {
                *cwd.size.borrow_mut() += size.parse::<usize>().unwrap();
            }
        };
    }

    let part_1_res = part_1(&mut root);
    let part_2_res = part_2(&mut root);

    println!("Part 1: {}", part_1_res);
    println!("Part 2: {}", part_2_res);
}

fn part_1(root: &mut Rc<Dir>) -> usize {
    let mut to_visit = vec![Rc::clone(root)];
    let mut res = 0;

    while let Some(dir) = to_visit.pop() {
        to_visit.extend(dir.subdir.borrow().values().map(Rc::clone));

        let size = dir.get_size();

        if size <= 100000 {
            res += size;
        }
    }

    res
}

fn part_2(root: &mut Rc<Dir>) -> usize {
    let cur_size = root.get_size();
    let free_size = 70000000 - cur_size;
    let need = 30000000 - free_size;

    let mut to_visit = vec![Rc::clone(root)];
    let mut res = usize::MAX;

    while let Some(dir) = to_visit.pop() {
        to_visit.extend(dir.subdir.borrow().values().map(Rc::clone));

        let size = dir.get_size();

        if size >= need {
            res = res.min(size);
        }
    }

    res
}
