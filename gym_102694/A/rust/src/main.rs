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
    // println!("{}", n);

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

    /*
    println!(
        "{:?}",
        tree
    );
     */

    if tree.is_empty() {
        println!("0");
        return;
    }

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
    let mut recent_lvls: Vec<Vec<i32>> = vec![vec![tree[0].0]];
    recent_lvls.push(vec![]);

    while !bfs.is_empty() {
        let x = bfs.pop_front().unwrap();
        vis.insert(x);
        // println!("just visited {}", x);
        let mut advance = false;
        for i in &tree {
            if i.0 == x && !vis.contains(
                &i.1
            ) {
                // println!("!val: {}", i.1);
                bfs.push_back(i.1);
                vis.insert(i.1);
                recent_lvls.last_mut().unwrap().push(i.1);
                advance = true;
            }
        }
        if advance == true {
            recent_lvls.push(vec![]);
        }
    }
    recent_lvls.pop();
    /*
    println!(
        "{:?}",
        recent_lvls
    );
     */
    // can get the last 2 levels
    let last = &recent_lvls[recent_lvls.len() - 1];
    let pre_last = &recent_lvls[recent_lvls.len() - 2];
    /*
    println!(
        "{:?}, {:?}",
        pre_last,
        last
    );
     */

    if last.len() > 1 {
        println!("{}",
                 (recent_lvls.len() - 1) * 2
        );
    } else {
        println!("{}",
                 (recent_lvls.len() - 1) * 2 - 1
        );
    }
}
