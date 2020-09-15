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
    let mut num_visited = leaves.len() as i32;
    let mut look_for_central_nodes = false;
    let mut central_nodes: Vec::<i32> = vec![];
    if num_visited > n as i32 - 3 {
        // println!("looking for central nodes now");
        look_for_central_nodes = true;
    }
    for l in &leaves {
        vis.insert(*l);
        bfs.push_back(*l);
    }
    let mut two_node_center = false;
    // println!("------");
    while !bfs.is_empty() {
        let current = bfs.pop_front().unwrap();
        for next in &tree[current as usize] {
            if !vis.contains(next) {
                // println!("{} -> {}", current, *next);
                if look_for_central_nodes == true {
                    central_nodes.push(*next);
                }
                num_visited += 1;
                vis.insert(*next);
                bfs.push_back(*next);
                if num_visited > n as i32 - 3 {
                    // println!("looking for central nodes now");
                    look_for_central_nodes = true;
                }
            }
        }
    }
    /*
    println!(
        "central nodes {:?}",
        central_nodes
    );
     */
    if central_nodes.len() == 2 {
        // we don't really need to know both of them
        two_node_center = true;
        central_nodes.pop();
    }
    vis = std::collections::HashSet::<i32>::new();
    bfs = std::collections::VecDeque::<i32>::new();
    let mut dist_nodes = std::collections::HashMap::<i32, i32>::new();
    for c in &central_nodes {
        bfs.push_back(*c);
        vis.insert(*c);
        dist_nodes.insert(*c, 0);
    }
    // this bfs should arrive at all leaves at the same time
    while !bfs.is_empty() {
        let current = bfs.pop_front().unwrap();
        if tree[current as usize].len() == 1 && vis.contains(&current) {
            /*
            println!(
                "stopped at step {}",
                dist_nodes[&current]
            );
             */

            if two_node_center == true {
                println!("{}", (dist_nodes[&current] * 2 + 1) * 3);
            } else {
                println!("{}", dist_nodes[&current] * 6);
            }
            break;
        }

        for next in &tree[current as usize] {
            if !vis.contains(next) {
                bfs.push_back(*next);
                vis.insert(*next);
                dist_nodes.insert(
                    *next,
                    dist_nodes[&current] + 1
                );
            }
        }
    }
}
