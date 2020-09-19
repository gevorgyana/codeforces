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

    // if we need the longest possible distance, then we might better
    // then it might probably make sense to start from a leaf. because
    // it makes no sense to start

    // find the leaves
    let mut leaves = std::collections::VecDeque::<i32>::new();
    let mut root = -1;
    for (i, val) in tree.iter().enumerate() {
        if !val.is_empty() {
            root = i as i32;
            break;
        }
    }
    let mut vis = std::collections::HashSet::<i32>::new();
    let mut bfs = std::collections::VecDeque::<i32>::new();
    bfs.push_back(root);
    vis.insert(root);
    while !bfs.is_empty() {
        let current = bfs.pop_front().unwrap();
        let mut is_leaf = true;
        for next in &tree[current as usize] {
            if !vis.contains(next) {
                is_leaf = false;
                vis.insert(*next);
                bfs.push_back(*next);
            }
        }
        if is_leaf == true {
            leaves.push_back(current);
        }
    }
    if tree[root as usize].len() == 1 {
        leaves.push_back(tree[root as usize][0]);
    }

    let mut longest_dist = i32::min_value();
    // then tranverse the whole tree starting from the leaves
    for root in &leaves {
        let mut vis = std::collections::HashSet::<i32>::new();
        let mut bfs = std::collections::VecDeque::<i32>::new();
        let mut lvl = std::collections::HashMap::<i32, i32>::new();
        bfs.push_back(*root);
        vis.insert(*root);
        lvl.insert(*root, 0);
        while !bfs.is_empty() {
            let current = bfs.pop_front().unwrap();
            longest_dist = longest_dist.max(lvl[&current]);
            for next in &tree[current as usize] {
                if !vis.contains(next) {
                    vis.insert(*next);
                    bfs.push_back(*next);
                    lvl.insert(
                        *next,
                        lvl[&current] + 1
                    );
                }
            }
        }
    }

    println!(
        "{}",
        longest_dist * 3
    );
}
