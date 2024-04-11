use crate::Stage;

pub fn solve(stage: Stage, input: Vec<String>) -> i128 {
    let mut sums = Vec::<u64>::new();
    let mut current_sum = 0u64;

    for row in input {
        if row.len() == 0 {
            sums.push(current_sum);
            current_sum = 0;
            continue;
        }
        current_sum += row.parse::<u64>().unwrap();
    }
    sums.push(current_sum);
    
    sums.sort_by_key(|x| u64::MAX - (*x));
    
    return sums.iter().take(match stage {
        Stage::Easy => {1}
        Stage::Hard => {3}
    }).sum::<u64>() as i128;
}
