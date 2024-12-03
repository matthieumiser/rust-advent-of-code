use std::fs::read_to_string;

fn main() {
    let file = read_to_string("day_1_input").unwrap();
    let combined_pairs: Vec<Vec<usize>> = file
        .lines()
        .map(|x| {
            x.split("   ")
                .map(|x| x.parse::<usize>().expect("expected a usize"))
                .collect()
        })
        .collect();
    let mut list_1: Vec<usize> = combined_pairs.iter().map(|x| x[0]).collect();
    list_1.sort();
    let mut list_2: Vec<usize> = combined_pairs.iter().map(|x| x[1]).collect();
    list_2.sort();

    let mut distance: usize = 0;
    for (a, b) in list_1.into_iter().zip(list_2) {
        distance += a.abs_diff(b);
    }
    println!("{:?}", distance);
}
