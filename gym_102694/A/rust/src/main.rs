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
    let mut tree: Vec<(i32, i32)> = vec![];
    for i in 0..n - 1 {
        // read the whole line
        let pair = read_line_of_i32s();
        tree.push((pair[0], pair[1]));
        tree.push((pair[1], pair[0]));
    }

    if n == 1 {
        println!("0");
        return;
    }

    /* Computationally no reason to sort unless you have upper/lower
     * bounds which I do not have in Rust, and I don't want to implement
     * that. So just use linear search thru the whole array, or allocate
     * memory in advance (which is another thing I don't want to do).
    tree.sort_by(
        |a, b| {
            a.0.cmp(&b.0)
        }
    );
     */

    // Run a bfs and identify leaves
    let mut bfs = std::collections::VecDeque::<i32>::new();
    let mut vis = std::collections::HashSet::<i32>::new();
    let mut leaves: Vec<i32> = vec![];
    let mut node_level = std::collections::HashMap::<i32, i32>::new();
    let root = tree[0].0;
    node_level.insert(root, 0);
    bfs.push_back(root);
    vis.insert(root);
    let mut level = 1;
    while !bfs.is_empty() {
        let current = bfs.pop_front().unwrap();
        let frozen_len = bfs.len();
        for c in &tree {
            if c.0 == current && !vis.contains(&c.1) {
                // println!("{} -> {}", current, c.1);
                vis.insert(c.1);
                bfs.push_back(c.1);
                node_level.insert(c.1, level);
                /*
                println!("assign {} to {}",
                         level, c.1
                );
                 */
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
