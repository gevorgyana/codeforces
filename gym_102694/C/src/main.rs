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
        let int = read_line_of_i32s();
        tree[int[0] as usize].push(int[1]);
        tree[int[1] as usize].push(int[0]);
    }
    // println!("{:?}", tree);
    let q_ = read_line_of_i32s()[0];
    for q in 0..q_ {
        let input = read_line_of_i32s();
        let a = input[0];
        let b = input[1];
        let c = input[2];
        // Calculate the distance to B, and remember the path to it.
        let mut vis = std::collections::HashSet::<i32>::new();
        let mut bfs = std::collections::VecDeque::<i32>::new();
        let mut lvl = std::collections::HashMap::<i32, i32>::new();
        let mut par = std::collections::HashMap::<i32, i32>::new();
        bfs.push_back(a);
        vis.insert(a);
        lvl.insert(a, 0);
        par.insert(a, -1);
        while !bfs.is_empty() {
            let current = bfs.pop_front().unwrap();
            if current == b {
                break;
            }
            for next in &tree[current as usize] {
                if !vis.contains(next) {
                    bfs.push_back(*next);
                    vis.insert(*next);
                    lvl.insert(
                        *next,
                        lvl[&current] + 1
                    );
                    par.insert(
                        *next,
                        current
                    );
                }
            }
        }

        /*
        println!(
            "parent {:?}",
            par
        );

        println!(
            "lvl {:?}",
            lvl
        );
         */

        if par.contains_key(&b) {
            if lvl[&b] < c {
                println!("{}", b);
            } else {
                let mut back = b;
                while
                    par.contains_key(&back) &&
                    par[&back] > 0 &&
                    lvl[&back] > c {
                        // println!("{}", back);
                        back = par[&back];
                    }
                println!("{}", back);
            }
        }
    }
}
