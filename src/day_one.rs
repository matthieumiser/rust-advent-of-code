use std::{collections::HashMap, fs::read_to_string, usize};

pub fn day_one() {
    let file = read_to_string("day_1_input").unwrap();
    let combined_pairs: Vec<Vec<usize>> = file
        .lines()
        .map(|x| {
            x.split("   ")
                .map(|x| x.parse::<usize>().expect("expected a usize"))
                .collect()
        })
        .collect();
    let mut mut_left: Vec<usize> = combined_pairs.iter().map(|x| x[0]).collect();
    mut_left.sort();
    let mut mut_right: Vec<usize> = combined_pairs.iter().map(|x| x[1]).collect();
    mut_right.sort();

    let left = mut_left.clone();
    let right = mut_right.clone();
    let left_occ = count_occurences(left);
    let right_occ = count_occurences(right);

    println!("Day 1 distance");
    println!("{:?}", calc_distance(mut_left, mut_right));
    println!("Day 1 similarity");
    println!("{:?}", calc_similarity(left_occ, right_occ));
}

fn count_occurences(input: Vec<usize>) -> HashMap<usize, usize> {
    let mut counts = HashMap::new();
    for i in input {
        let count = counts.entry(i).or_insert(0);
        *count += 1
    }

    return counts;
}

fn calc_similarity(left: HashMap<usize, usize>, right: HashMap<usize, usize>) -> usize {
    let mut similarity: usize = 0;
    for (key, value) in left {
        similarity += if right.contains_key(&key) {
            key * right[&key] * value
        } else {
            0
        };
    }
    return similarity;
}

fn calc_distance(left: Vec<usize>, right: Vec<usize>) -> usize {
    let mut distance: usize = 0;
    for (a, b) in left.into_iter().zip(right) {
        distance += a.abs_diff(b);
    }

    return distance;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_occurences() {
        let test_case = vec![3, 4, 2, 1, 3, 3];
        let result = count_occurences(test_case);
        let expected = HashMap::from([(3, 3), (4, 1), (2, 1), (1, 1)]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_calc_similarity() {
        let left = HashMap::from([(3, 3), (4, 1), (2, 1), (1, 1)]);
        let right = HashMap::from([(4, 1), (3, 3), (5, 1), (9, 1)]);
        let result = calc_similarity(left, right);
        assert_eq!(result, 31);
    }
}
