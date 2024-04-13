use std::cmp::max;
use std::iter;

use crate::Stage;

pub fn solve(stage: Stage, input: &Vec<&str>) -> String {
    match stage {
        Stage::Easy => count_visible(input),
        Stage::Hard => find_best_spot(input),
    }
    .to_string()
}

// yeah, it's slow. But who cares? O(N^3) is O(1) if N is constant :-p
fn find_best_spot(tree_map: &[&str]) -> usize {
    fn count_visible(
        tree_map: &[&str],
        height: u8,
        i_range: impl IntoIterator<Item = usize> + Clone,
        j_range: impl IntoIterator<Item = usize> + Clone,
    ) -> usize {
        let mut cnt = 0;

        for i in i_range {
            for j in j_range.clone() {
                if tree_map[i].as_bytes()[j] < height {
                    cnt += 1;
                } else {
                    return cnt + 1
                }
            }
        }

        cnt
    }

    let mut best = 0;

    for i in 1..tree_map.len() - 1 {
        for j in 1..tree_map[i].len() - 1 {
            let height = tree_map[i].as_bytes()[j];
            let left = count_visible(tree_map, height, i..i + 1, (0..j).rev());
            let right = count_visible(tree_map, height, i..i + 1, j + 1..tree_map[i].len());
            let up = count_visible(tree_map, height, (0..i).rev(), j..j + 1);
            let down = count_visible(tree_map, height, i + 1..tree_map.len(), j..j + 1);

            best = max(best, left * right * up * down)
        }
    }

    best
}

// yeah, it could be more DRY. But who cares?
fn count_visible(tree_map: &[&str]) -> usize {
    let mut visibility_map: Vec<Vec<bool>> = tree_map
        .iter()
        .map(|row| iter::repeat(false).take(row.len()).collect())
        .collect();

    for i in 0..tree_map.len() {
        let mut current_height = 0;
        for j in 0..tree_map[i].len() {
            let height = tree_map[i].as_bytes()[j];

            if height > current_height {
                visibility_map[i][j] = true;
            }
            current_height = max(current_height, height);
        }
    }

    for i in 0..tree_map.len() {
        let mut current_height = 0;
        for j in (0..tree_map[i].len()).rev() {
            let height = tree_map[i].as_bytes()[j];

            if height > current_height {
                visibility_map[i][j] = true;
            }
            current_height = max(current_height, height);
        }
    }

    for j in 0..tree_map[0].len() {
        let mut current_height = 0;
        for i in 0..tree_map.len() {
            let height = tree_map[i].as_bytes()[j];

            if height > current_height {
                visibility_map[i][j] = true;
            }
            current_height = max(current_height, height);
        }
    }

    for j in 0..tree_map[0].len() {
        let mut current_height = 0;
        for i in (0..tree_map.len()).rev() {
            let height = tree_map[i].as_bytes()[j];

            if height > current_height {
                visibility_map[i][j] = true;
            }
            current_height = max(current_height, height);
        }
    }

    visibility_map
        .iter()
        .map(|r| r.iter().filter(|x| **x).count())
        .sum()
}
