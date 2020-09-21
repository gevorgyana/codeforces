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
        let integers = read_line_of_i32s();
        tree[integers[0] as usize].push(integers[1]);
        tree[integers[1] as usize].push(integers[0]);
    }

    // run the graph traversal and mark diametral nodes
    let mut vis = std::collections::HashSet::<i32>::new();
    let mut bfs = std::collections::VecDeque::<i32>::new();
    let mut lvl = std::collections::HashMap::<i32, i32>::new();
    let root = 1;
    bfs.push_back(root);
    vis.insert(root);
    lvl.insert(root, 0);

    while !bfs.is_empty() {
        let current = bfs.pop_front().unwrap();
        /*
        println!(
            "visited {} at lvl {}",
            current,
            lvl[&current]
        );
         */
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

    let mut max_lvl: i32 = 0;
    for i in &lvl {
        max_lvl = max_lvl.max(*i.1);
    }

    let mut diam_leaves = std::collections::HashSet::<i32>::new();
    let mut max_lvl_nodes = Vec::<i32>::new();
    for i in &lvl {
        if max_lvl == *i.1 {
            // println!("node {}", i.0);
            max_lvl_nodes.push(*i.0);
            diam_leaves.insert(*i.0);
        }
    }

    let mut vis = std::collections::HashSet::<i32>::new();
    let mut bfs = std::collections::VecDeque::<i32>::new();
    let mut lvl = std::collections::HashMap::<i32, i32>::new();
    for r in &max_lvl_nodes {
        lvl.insert(*r, 0);
        bfs.push_back(*r);
        vis.insert(*r);
    }

    while !bfs.is_empty() {
        let current = bfs.pop_front().unwrap();
        /*
        println!(
            "visited {} at lvl {}",
            current,
            lvl[&current]
        );
         */
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

    let mut max_lvl: i32 = 0;
    for i in &lvl {
        max_lvl = max_lvl.max(*i.1);
    }

    for i in &lvl {
        if max_lvl == *i.1 {
            diam_leaves.insert(*i.0);
        }
    }

    /*
    println!(
        "diam leaves {:?}",
        diam_leaves
    );
     */

    for i in 1..n+1 {
        if diam_leaves.contains(&i) {
            println!(
                "{}", max_lvl + 1
            );
        } else {
            println!(
                "{}", max_lvl
            );
        }
    }
}
