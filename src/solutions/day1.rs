use crate::Stage;

pub fn solve(stage: Stage, input: &Vec<String>) -> String {
    let mut sums = Vec::<i64>::new();
    let mut current_sum = 0i64;

    for row in input {
        if row.len() == 0 {
            sums.push(current_sum);
            current_sum = 0;
            continue;
        }
        current_sum += row.parse::<i64>().unwrap();
    }
    sums.push(current_sum);

    sums.sort_by_key(|x| -(*x));

    let result: i64 = sums
        .iter()
        .take(match stage {
            Stage::Easy => 1,
            Stage::Hard => 3,
        })
        .sum();

    result.to_string()
}
