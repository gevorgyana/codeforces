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

    let mut root = -1;
    for (i, val) in tree.iter().enumerate() {
        if !val.is_empty() {
            root = i as i32;
            break;
        }
    }

    let mut vis = std::collections::HashSet::<i32>::new();
    let mut bfs = std::collections::VecDeque::<i32>::new();
    let mut lvl = std::collections::HashMap::<i32, i32>::new();
    vis.insert(root);
    bfs.push_back(root);
    lvl.insert(root, 0);
    let mut furthest_node: (i32, i32) = (-1, -1);
    while !bfs.is_empty() {
        let current = bfs.pop_front().unwrap();
        // println!("{} with lvl {}", current, lvl[&current]);
        for next in &tree[current as usize] {
            if !vis.contains(next) {
                bfs.push_back(*next);
                vis.insert(*next);
                lvl.insert(
                    *next,
                    lvl[&current] + 1
                );
                if furthest_node.0 < lvl[next] {
                    furthest_node.0 = lvl[next];
                    furthest_node.1 = *next;
                }
            }
        }
    }

    root = furthest_node.1;

    let mut vis = std::collections::HashSet::<i32>::new();
    let mut bfs = std::collections::VecDeque::<i32>::new();
    let mut lvl = std::collections::HashMap::<i32, i32>::new();
    vis.insert(root);
    bfs.push_back(root);
    lvl.insert(root, 0);
    let mut ans = 0;
    while !bfs.is_empty() {
        let current = bfs.pop_front().unwrap();
        ans = ans.max(lvl[&current]);
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
    println!(
        "{}",
        ans * 3
    );
}
