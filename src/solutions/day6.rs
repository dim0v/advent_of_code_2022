use crate::Stage;

pub fn solve(stage: Stage, input: &Vec<String>) -> String {
    let input = &input[0];

    find_sop_marker(
        input,
        match stage {
            Stage::Easy => 4,
            Stage::Hard => 14,
        },
    )
    .to_string()
}

fn find_sop_marker(s: &String, cnt: usize) -> usize {
    let mut buf: Vec<char> = s.chars().take(cnt).collect();
    let mut head = 0;

    for (i, c) in s.chars().skip(cnt).enumerate() {
        if is_all_unique(&buf) {
            return i + cnt;
        }

        buf[head] = c;
        head = (head + 1) % cnt;
    }

    s.len()
}

fn is_all_unique(chars: &Vec<char>) -> bool {
    for (i, c1) in chars.iter().enumerate() {
        for c2 in chars.iter().skip(i + 1) {
            if c1 == c2 { return false }
        }
    }

    true
}
