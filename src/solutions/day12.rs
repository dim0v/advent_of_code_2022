use std::cmp::min;
use std::collections::VecDeque;
use std::iter;

use crate::Stage;

pub fn solve(stage: Stage, input: &Vec<&str>) -> String {
    let (full, nearest) = bfs(input);

    (match stage {
        Stage::Easy => full,
        Stage::Hard => nearest,
    })
    .to_string()
}

fn bfs(map: &[&str]) -> (usize, usize) {
    let mut dist_map: Vec<Vec<Option<usize>>> = map
        .iter()
        .map(|row| iter::repeat(None).take(row.len()).collect())
        .collect();
    let (start, end) = extract_endpoints(map);
    let mut q = VecDeque::with_capacity(16);
    q.push_back((0usize, start));

    while let Some((d, (i, j))) = q.pop_front() {
        let dist = &mut dist_map[i][j];
        if dist.is_some() {
            continue;
        }
        *dist = Some(d);

        let height = map[i].as_bytes()[j];

        if end == (i, j) {
            break;
        }

        let height = fix_height(height);

        let potential_neighbors = [
            (i as isize - 1, j as isize),
            (i as isize + 1, j as isize),
            (i as isize, j as isize - 1),
            (i as isize, j as isize + 1),
        ];

        for (n_i, n_j) in potential_neighbors {
            if n_i < 0
                || n_j < 0
                || n_i >= dist_map.len() as isize
                || n_j >= dist_map[0].len() as isize
            {
                continue;
            }

            let (n_i, n_j) = (n_i as usize, n_j as usize);

            let n_dist = &dist_map[n_i][n_j];
            if n_dist.is_some() {
                continue;
            }

            let n_height = fix_height(map[n_i].as_bytes()[n_j]);
            if height - n_height > 1 {
                continue;
            }

            q.push_back((d + 1, (n_i, n_j)));
        }
    }

    let result_dist = dist_map[end.0][end.1].unwrap();
    let lowest_a = {
        let mut result = usize::MAX;
        for (row_height, row_dist) in map.iter().zip(dist_map) {
            for (height, dist) in row_height.as_bytes().iter().zip(row_dist) {
                if *height != b'a' {
                    continue;
                }
                if let Some(dist) = dist {
                    result = min(result, dist)
                }
            }
        }
        result
    };

    (result_dist, lowest_a)
}

fn fix_height(h: u8) -> i8 {
    (match h {
        b'S' => b'a',
        b'E' => b'z',
        x => x,
    }) as i8
}

fn extract_endpoints(map: &[&str]) -> ((usize, usize), (usize, usize)) {
    let mut start: Option<(usize, usize)> = None;
    let mut end: Option<(usize, usize)> = None;

    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.as_bytes().iter().enumerate() {
            if *c == b'E' {
                start = Some((i, j));
            }
            if *c == b'S' {
                end = Some((i, j));
            }
        }
    }

    (start.unwrap(), end.unwrap())
}
