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
    let mut nodes = std::collections::HashSet::<i32>::new();
    for i in 0..n - 1 {
        // read the whole line
        let pair = read_line_of_i32s();
        tree[pair[0] as usize].push(pair[1]);
        tree[pair[1] as usize].push(pair[0]);
        nodes.insert(
            pair[0]
        );
        nodes.insert(
            pair[1]
        );
    }
    if n == 1 {
        println!("0");
        return;
    }

    // locate the leaves of the tree,
    // then locate the center of the tree
    // then move from the center to the leaves, and calculate the distance
    // we have covered all the paths - so if we sum the

    // diameter is the longest thing - if we start from the center, we
    // minimize the distances. diameter is the maximum distance. so
    // we need a metric contrary to the central distance. central distance
    // is the minimum distance.
}
