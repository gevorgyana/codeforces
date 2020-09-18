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
        // println!("root is a leaf too");
        leaves.push_back(
            root
        );
    }
    /*
    println!("leaves {:?}",
             leaves
    );
     */

    // second pass
    // find the center, then move from the center to the leaves and
    // measure the distance, then multiply it by 2

    let mut bfs = std::collections::VecDeque::<i32>::new();
    let mut central_nodes: Vec::<i32> = vec![];
    let mut bfs_level = std::collections::HashMap::<i32, i32>::new();
    for leaf in &leaves {
        bfs.push_back(*leaf);
        vis.insert(*leaf);
        bfs_level.insert(
            *leaf, 0
        );
    }

    // collect central nodes
    while !bfs.is_empty() {
        let current = bfs.pop_front().unwrap();
        // println!("vis {} at lvl {}", current, bfs_level[&current]);
        // update the best level
        for next in &tree[current as usize] {
            if vis.contains(next) {
                continue
            }
            // println!("discover {} -> {}", current, next);
            bfs.push_back(*next);
            vis.insert(*next);
            bfs_level.insert(
                *next,
                bfs_level[&current] + 1
            );
        }
    }

    // collect the central nodes
    let max_level
        = bfs_level.values().max().unwrap();
    for i in &bfs_level {
        if i.1 == max_level {
            central_nodes.push(*i.0);
        }
    }

    assert!(central_nodes.len() < 3);

    /*
    println!(
        "central nodes {:?}",
        central_nodes
    );
     */

    // todo1 most probably, the problem is that i never looked at # adjacent nodes
    // that the central nodes have.
    // todo2 - calculate the diameter as the longest path and compare with the thngs
    // u obtained from the central walk - they must be equal to each other.
    if central_nodes.len() == 1 {
        println!("{}",
                 (max_level * 2) * 3
        );
    } else {
        println!("{}",
                 (max_level * 2 + 1) * 3
        );
    }
}
