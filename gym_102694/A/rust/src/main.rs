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
    let mut tree: Vec<Vec<i32>> = vec![vec![]; 3 * 100000 + 1];
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
    let mut leaves: Vec<i32> = vec![];
    let mut node_level = std::collections::HashMap::<i32, i32>::new();
    let mut root = -1;
    for i in &tree {
        if !i.is_empty() {
            root = i[0];
            break;
        }
    }
    node_level.insert(root, 0);
    bfs.push_back(root);
    vis.insert(root);
    let mut level = 1;

    while !bfs.is_empty() {
        let current = bfs.pop_front().unwrap();
        let frozen_len = bfs.len();
        for c in &tree[current as usize] {
            println!(
                "{} -> {}",
                current, c
            );
            if !vis.contains(&c) {
                vis.insert(*c);
                bfs.push_back(*c);
                node_level.insert(*c, level);
            }
        }

        if bfs.len() == frozen_len {
            leaves.push(current);
        }
        level += 1;
    }
    /*
    println!(
        "{:?}",
        leaves
    );
    println!("{:?}",
             node_level
    );
     */

    let mut max_dist: i32 = 0;
    let mut penmax_dist: i32 = 0;
    for l in &leaves {
        // println!("leave {}, level {}", l, node_level[l]);
        // store only the last 2 maximal values
        let mut temp_vec = vec![
            node_level[l], penmax_dist, max_dist,
        ];
        temp_vec.sort();
        penmax_dist = temp_vec[1];
        max_dist = temp_vec[2];
    }

    println!(
        "{}",
        (penmax_dist + max_dist) * 3
    );

}
