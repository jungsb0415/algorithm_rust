
fn print_picked(picked: Vec<i32>) -> () {
    for i in picked {
        print!("{i} ");
    }
    print!("\n");
}

// An algorithm to find all combinations of choosing m elements from n elements
pub fn pick(n: i32, mut picked: Vec<i32>, to_pick: i32) -> () {
    if to_pick == 0 {
        print_picked(picked.clone());
        return;
    }

    let smallest = picked.last().copied().unwrap_or(0);

    for next in smallest..n {
        picked.push(next);
        pick(n, picked.clone(), to_pick - 1);
        picked.pop();
    }
}

