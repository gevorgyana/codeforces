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
    println!("{}", n);
    let mut tree: Vec<(i32, i32)> = vec![];
    for i in 0..n - 1 {
        // read the whole line
        let pair = read_line_of_i32s();
        tree.push(
            (
                pair[0],
                pair[1]
            )
        );
    }
    tree.sort_by(
        |a, b| {
            a.0.cmp(&b.0)
        }
    );
    if tree.is_empty() {
        println!("0");
        return;
    }
    println!(
        "{:?}",
        tree
    );
    let mut bfs: std::collections::VecDeque::<i32> =
        std::collections::VecDeque::new();
    let mut vis: std::collections::HashSet::<i32> =
        std::collections::HashSet::new();
    vis.insert(
        tree[0].0
    );
    bfs.push_back(
        tree[0].0
    );
    let mut level: i32 = 0;
    let mut parent = std::collections::HashMap::<i32, i32>::new();
    while !bfs.is_empty() {
        let x = bfs.pop_front().unwrap();
        vis.insert(x);
        for i in &tree {
            if i.0 == x && !vis.contains(
                &i.1
            ) {
                bfs.push_back(i.1);
                vis.insert(i.1);
                parent.insert(
                    i.1,
                    x,
                );
            }
        }
    }
    println!(
        "{:?}",
        parent
    );
    vis = std::collections::HashSet::new();
    let mut reach = std::collections::HashMap::<i32, i32>::new();
    for (start_from, _) in &tree {
        println!(
            "started from {}",
            start_from
        );
        let mut e = *start_from;
        let mut trail: Vec<i32> = vec![];
        while parent.contains_key(&e) && !vis.contains(&e) {
            trail.push(e);
            vis.insert(e);
            e = parent[&e];
        }
        println!(
            "reach {:?}",
            reach
        );
        println!("the trail {:?}", trail);

        // if root
        if trail.len() == 0 {
            reach.insert(e, 0);
        }

        // traverse the trail & fill in the gaps
        for (ind, rval) in trail.iter().rev().enumerate() {
            println!(
                "ind {} : rval {}",
                ind, rval
            );
            if !reach.contains_key(rval) {
                reach.insert(*rval, 0);
            }
            println!(
                "adding {} and {} to {}",
                ind + 1,
                reach[&parent[trail.last().unwrap()]],
                rval,
            );
            *reach.get_mut(rval).unwrap()
                += reach[&parent[trail.last().unwrap()]]
                + ind as i32 + 1;
        }
    }
    println!(
        "{:?}",
        reach
    );

    println!(
        "the reach {:?}",
        reach
    );

    let capacity = reach.iter().len();
    if capacity > 1 {
        println!(
            "{}",
            reach.values().nth(capacity - 1)
                + reach.values().nth(capacity - 2)
        );
    } else {
        println!(
            "{}",

        );
    }
}
