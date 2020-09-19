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
    let mut best_ans = i32::min_value();
    for root in nodes {
        let mut bfs = std::collections::VecDeque::<i32>::new();
        let mut vis = std::collections::HashSet::<i32>::new();
        let mut lvl = std::collections::HashMap::<i32, i32>::new();
        bfs.push_back(root);
        vis.insert(root);
        lvl.insert(root, 0);
        while !bfs.is_empty() {
            let current = bfs.pop_front().unwrap();
            let current_lvl = lvl[&current];
            // println!("current {} : lvl {}", current, current_lvl);
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
        /*
        println!(
            "{:?}",
            lvl
        );
         */
        /*
        let max_level_per_simple_algorithm
            = lvl.values().max();
        println!("{}", *max_level_per_simple_algorithm.unwrap() * 3);
         */
        best_ans = best_ans.max(*lvl.values().max().unwrap() * 3);
    }

    println!(
        "{}",
        best_ans
    );

    /*
    the diameter is the longest path between any 2 nodes
    we can use the defininton. take every node, and run bfs from it
    so in N ^ 2 time we will find the answer by contantly updating it.
    we could also find just the central nodes, and from them walk all
    the way down the tree. would it work? if we have a center and we run
    from the center, then it should work. let's check that.
     */

}
