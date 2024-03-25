use aoc_runner_derive::{aoc, aoc_generator};

// struct Directory {
//     name: String,
//     children: Vec<Node>,
// }

// impl Directory {
//     fn size(&self) -> usize {
//         if self.children.len() == 0 {
//             0
//         } else {
//             self.children.iter().map(|s| s.size()).sum::<usize>()
//         }
//     }
// }

// enum Node {
//     File(File),
//     Directory(Directory),
// }

// struct File {
//     name: String,
//     size: usize,
// }

// impl File {
//     fn size(&self) -> usize {
//         self.size
//     }
// }

#[derive(Clone)]
struct Node {
    name: String,
    size: usize,
    children: Vec<Node>,
}

impl Node {
    // Assumes that:
    // * All commands are legitmate.
    // * All sub-directories wll be entered in turn.
    // * Each `cd` command will only be followed by another `cd` command or an
    //   `ls` command.
    fn from_dir(name: String, children: Vec<Node>) -> Node {
        let size: usize = children.iter().map(|i| i.size).sum();

        Node { name: name, size: size, children: children }
    }
    fn from_file(name: String, size: usize) -> Node {
        Node { name: name, size: size, children: vec!() }
    }

    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    fn from_inputs<'a, I>(name: String, input: &mut I) -> Node
    where
        I: Iterator<Item = &'a str>,
    {
        let mut result: Node = Node {
            name: name,
            size: 0, // initialises as empty directory.
            children: vec![]
        };

        let mut list: Vec<String> = vec![];

        while let Some(line) = input.next() {
            if line.starts_with("$ cd") {
                // Start with matching directory.

                let child_name: String = line
                .split(' ')
                .collect::<Vec<&str>>()[2]
                .to_owned();

                if child_name == ".." {
                    return result
                }
                
                // Check if child directory is in list, if not, warn. 
                if !list.contains(&child_name) {
                    println!(
                        "WARNING: {} list does not contain {}",
                        result.name,
                        child_name,
                    );
                }

                result.children.push(Node::from_inputs(child_name, input));

            } else if line.split(' ').collect::<Vec<&str>>().len() == 2 {
                if line.starts_with("dir") {
                    // Handle case of directory.

                    list.push(
                        line.split(' ').collect::<Vec<&str>>()[1].to_owned()
                    );
                } else {
                    let v: Vec<&str> = line.split(' ').collect();

                    if let Ok(filesize) = v[0].parse::<usize>() {
                        // First field is a number.
                        let filename: String = v[1].to_owned();

                        result.add_child(
                            Node {
                                name: filename,
                                size: filesize,
                                children: vec![],
                            }
                        )
                    }
                }
            } else {
                // Handle case where input is not expected.
                panic!("Unexpected input: {}", line);
            }

            result.size = result
                    .children
                    .iter()
                    .map(|c| c.size)
                    .sum();
        }

        result
    }
}

fn total_size_of_children_dir_under(
    node: Node,
    target: usize,
) -> Vec<usize> {
    /// Returns `vec` of sizes of children of `node` where size < `target`.
    /// 
    /// If `node` has no children, then it should return a `vec` with only one
    /// item: the size of `node`, or 0. Otherwise, it should return a `vec`
    /// with a `vec` of the size of the results of itself called recursively on
    /// its children, and the size of `node` if it's larger than `target`, or
    /// 0.
    let mut list: Vec<usize> = vec![];

    if !node.children.is_empty() {
        // Is a directory
        let child_results: Vec<usize> = node
        .children
        .iter()
        .map(|c| total_size_of_children_dir_under(c.clone(), target))
        .flatten()
        .collect();
        
        list.extend(child_results);

        if node.size <= target {
            list.push(node.size);
        }
    }
    list
}

#[aoc_generator(day7)]
fn part1_input(input: &str) -> Node {
    let first_line = input.lines().next().expect("Input is empty!");

    if first_line != "$ cd /" {
        panic!("First line is not \"$ cd /\"");
    }

    println!("Processing input...");

    Node::from_inputs("/".to_owned(), &mut input.lines().skip(1))

    // for line in input.lines().skip(1) {
    //     if line.starts_with("$ cd") {
    //         // start with matching directory
    //         if line.split(' ').collect::<Vec<&str>>().len() == 3 {
    //             let dir = line.split(' ').collect()[2];
    //             if dir = ".." {} else if dir = "/" {
    //                 panic!("Second root directory encountered!");
    //             } else {}
    //         }
    //     } else if line.starts_with("dir ") {
    //         // handle directory
    //     } else {
    //         // Other cases, including starting with filesize.
    //         // Deliberate choice to put filesize in here so no need to resort
    //         // to regex.
    //         if line.split(' ').collect::<Vec<&str>>.len() == 2 {
    //             let v: Vec<&str> = line.split(' ').collect();
    //             if let Some(filesize) = v[0].parse::<usize>().unwrap() {
    //                 let filename: &str = v[1];
    //             }
    //         }
    //     }
    // }
}

#[aoc(day7, part1, mine)]
fn part1(dir: &Node) -> usize {
    let sample_input = "$ ls
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
7214296 k";
    
    let sample_node: Node = Node::from_inputs("/".to_owned(), &mut sample_input.lines());
    let sample_vector: Vec<usize> = total_size_of_children_dir_under(sample_node, 100000);
    let sample_result: usize = sample_vector.iter().sum();
    
    println!("Sample vector: {:?}", sample_vector);
    println!("Sample node result: {}", sample_result);

    total_size_of_children_dir_under(dir.clone(), 100000).iter().sum()
}