use std::{fs::read_to_string, i64, path::absolute};

pub fn day_two() {
    let output = file_to_int_vec("day_2_input");
    let mut num_safe = 0;

    for i in output {
        if is_safe(i) {
            num_safe += 1;
        }
    }
    println!("Number of safe reports: {}", num_safe);
}

fn file_to_int_vec(file_path: &str) -> Vec<Vec<i64>> {
    let file = read_to_string(file_path).unwrap();
    let vectorized: Vec<Vec<i64>> = file
        .lines()
        .map(|x| {
            x.split(" ")
                .map(|y| y.parse::<i64>().expect("expected an i64"))
                .collect()
        })
        .collect();
    return vectorized;
}

fn is_safe(levels: Vec<i64>) -> bool {
    let mut maybe_up_trend: Option<bool> = None;
    let mut maybe_last_num: Option<i64> = None;

    for i in levels {
        match maybe_last_num {
            None => {
                maybe_last_num = Some(i);
                continue;
            }
            Some(last_num) => {
                if (last_num - i).abs() > 3 || (last_num - i) == 0 {
                    return false;
                } else {
                    match maybe_up_trend {
                        None => maybe_up_trend = Some(i - last_num > 0),
                        Some(up_trend) => {
                            if (i - last_num < 0 && up_trend) || (i - last_num > 0 && !up_trend) {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        maybe_last_num = Some(i)
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe() {
        let input = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let result: Vec<bool> = input.into_iter().map(|x| is_safe(x)).collect();
        let expected = vec![true, false, false, false, false, true];
        assert_eq!(result, expected);
    }
}
