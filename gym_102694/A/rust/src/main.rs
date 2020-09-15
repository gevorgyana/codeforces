use std::io;
use std::collections;

fn read_line_of_i32s() -> Vec<i32> {
    let mut worker_line = String::new();
    io::stdin()
        .read_line(&mut worker_line)
        .unwrap();
    worker_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_of_i32s()[0];
    // it ugly but what can i do? i do not have the input in advance.
    let mut tree: Vec<Vec<i32>> = vec![vec![]; n as usize + 1];
    for i in 0..n - 1 {
        // read the whole line
        let pair = read_line_of_i32s();
        tree[pair[0] as usize].push(pair[1]);
        tree[pair[1] as usize].push(pair[0]);
    }
    if n == 1 {
        println!("0");
        return;
    }
    // Run a bfs and identify leaves
    let mut bfs = std::collections::VecDeque::<i32>::new();
    let mut vis = std::collections::HashSet::<i32>::new();
    // will later be used as a queue => therefore VecDeque
    let mut leaves = std::collections::VecDeque::<i32>::new();
    let mut root = -1;
    for (i, val) in tree.iter().enumerate() {
        if !val.is_empty() {
            root = i as i32;
            break;
        }
    }
    bfs.push_back(root);
    vis.insert(root);
    while !bfs.is_empty() {
        let current = bfs.pop_front().unwrap();
        let frozen_len = bfs.len();
        for c in &tree[current as usize] {
            if !vis.contains(&c) {
                vis.insert(*c);
                bfs.push_back(*c);
            }
        }
        if bfs.len() == frozen_len {
            leaves.push_back(current);
        }
    }
    vis = std::collections::HashSet::new();
    if tree[root as usize].len() == 1 {
        leaves.push_back(
            root
        );
    }
    println!("leaves {:?}",
             leaves
    );

    // second pass

    let mut num_visited: i32 = 0;
    let mut leaf_path = std::collections::HashMap::<i32, i32>::new();
    let mut collect_center: bool = false;
    let mut diam_leaves: Vec<i32> = vec![0, 0];

    for l in &leaves {
        vis.insert(*l);
        leaf_path.insert(*l, 0);
    }
    num_visited += leaves.len() as i32;
    if num_visited > n - 3 { collect_center = true; }

    while !leaves.is_empty() {
        let current = leaves.pop_front().unwrap();
        for next in &tree[current as usize] {
            if !vis.contains(&next) {

                leaf_path.insert(*next,
                                 leaf_path[&current]
                );

                if collect_center == true {
                    diam_leaves.push(leaf_path[&current]);
                    println!("the size of the vec {}", diam_leaves.len());
                    diam_leaves.swap(0, 2);
                    diam_leaves.pop();
                }

                vis.insert(*next);
                num_visited += 1;
                bfs.push_back(*next);
                println!("{} -> {}", current, next);

                if num_visited > n - 3 {
                    // the next nodes are going to be central
                    collect_center = true;
                }

            }
        }
    }
    vis = std::collections::HashSet::new();
    println!("{}", (diam_leaves[0] + diam_leaves[1]) * 3);
}
