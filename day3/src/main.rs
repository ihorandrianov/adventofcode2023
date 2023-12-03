use std::fs::read_to_string;

fn main() {
    let mut res: usize = 0;
    let file = read_to_string("input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(x, y)| (x, y.char_indices().collect::<Vec<(usize, char)>>()))
        .collect::<Vec<(usize, Vec<(usize, char)>)>>();

    file.iter().for_each(|(line, content)| {
        content.iter().for_each(|(i, ch)| {
            if ch == &'*' {
                let num = collect_adj_nums(i, line, content, &file);
                res += num
            }
        })
    });

    println!("{}", res)
}

fn collect_adj_nums(
    line_idx: &usize,
    col_idx: &usize,
    line: &Vec<(usize, char)>,
    rows: &Vec<(usize, Vec<(usize, char)>)>,
) -> usize {
    let look_left_val: usize = look_left(line_idx, line).parse().unwrap_or_default();
    let look_right_val: usize = look_right(line_idx, line).parse().unwrap_or_default();

    let top_row = &rows[col_idx - 1].1[line_idx - 3..=line_idx + 3];
    let mut look_top = look_row(top_row);

    let bottom_row = &rows[col_idx + 1].1[line_idx - 3..=line_idx + 3];
    let mut look_bottom = look_row(bottom_row);

    look_top.append(&mut look_bottom);
    look_top.push(look_left_val);
    look_top.push(look_right_val);

    let look_top = look_top
        .into_iter()
        .filter(|x| x != &0)
        .collect::<Vec<usize>>();
    if look_top.len() == 2 {
        return look_top.into_iter().reduce(|acc, x| acc * x).unwrap_or(0);
    } else {
        return 0;
    }
}

fn look_right(line_idx: &usize, line: &[(usize, char)]) -> String {
    let mut result = String::new();
    let mut right = line_idx + 1;
    while right < line.len() && line[right].1.is_numeric() {
        result.push(line[right].1);
        right += 1
    }
    result
}

fn look_left(line_idx: &usize, line: &[(usize, char)]) -> String {
    let mut result = String::new();
    let mut left = line_idx - 1;
    while line[left].1.is_numeric() {
        result.insert(0, line[left].1);
        if left != 0 {
            left -= 1
        } else {
            break;
        }
    }
    result
}

fn look_row(target_row: &[(usize, char)]) -> Vec<usize> {
    let mut res = vec![];
    let is_middle_numeric = target_row[target_row.len() / 2].1.is_numeric();

    match is_middle_numeric {
        true => {
            let mut left = look_left(&3, target_row);
            let right = look_right(&3, target_row);
            left.push(target_row[3].1);
            left.push_str(&right);
            res.push(left);
        }

        false => {
            let left = look_left(&3, target_row);
            let right = look_right(&3, target_row);
            res.push(left);
            res.push(right);
        }
    };
    res.iter()
        .map(|x| x.parse::<usize>().unwrap_or_default())
        .collect()
}
