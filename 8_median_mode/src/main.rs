use std::collections::HashMap;

fn median_finder(mut list: Vec<i64>) -> i64 {
    list.sort();

    list[list.len()/2]
}

fn mode_finder(list: &Vec<i64>) -> Option<i64> {
    let mut linker: HashMap<i64, i64> = HashMap::new();

    for i in list {
        let count = linker.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut max_appear: i64 = 0;
    let mut ans: Option<i64> = None;

    for (num, cnt) in &linker {
        if *cnt > max_appear {
            max_appear = *cnt;
            ans = Some(*num);
        }
    }

    ans
}

fn main() {
    let list: Vec<i64> = vec![-4, 7, 1, -3, 8, -1, 6, 2, -9, 4, 1, -2, 9, 1, -6, 7, -1, 41, -3];

    println! ("The list consists of these numbers:");
    print!("[ ");
    for i in &list {
        print!("{}, ", i);
    }
    println!("]");

    let mode: Option<i64> = mode_finder(&list);
    let median: i64 = median_finder(list);

    let mode: i64 = mode.unwrap_or_default();

    println!("The median of the numbers is: {}", median);
    println!("The mode of the numbers is: {}", mode);
}
