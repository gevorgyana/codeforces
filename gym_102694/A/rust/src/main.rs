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
        for next in &tree[current as usize] {
            if vis.contains(next) {
                continue
            }
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

    // now try to just run a bfs and see the maximum level that you can
    // get - and assert that your solution does the same.
    // println!("---");
    let mut bfs = std::collections::VecDeque::<i32>::new();
    let mut vis = std::collections::HashSet::<i32>::new();
    let mut lvl = std::collections::HashMap::<i32, i32>::new();
    bfs.push_back(root);
    vis.insert(root);
    lvl.insert(root, 0);
    while !bfs.is_empty() {
        let current = bfs.pop_front().unwrap();
        for next in &tree[current as usize] {
            if !vis.contains(next) {
                bfs.push_back(*next);
                vis.insert(*next);
                lvl.insert(
                    *next,
                    lvl[&current] + 1
                );
            }
        }
    }
    let mut ans: i32 = 0;
    if central_nodes.len() == 1 {
        ans = (max_level * 2) * 3;
    } else {
        ans = (max_level * 2 + 1) * 3;
    }
    // compare with the max level that you obtained from the other
    // algorithm - it can be optimized, but nevermind
    let max_level_per_simple_algorithm
        = lvl.values().max();
    assert!(/*ans == *max_level_per_simple_algorithm.unwrap() * 3
            &&*/
            (ans - *max_level_per_simple_algorithm.unwrap() * 3).abs() < 100000
    );
    println!("{}", ans);
}
