pub fn solve_board(minefield: &[&str]) -> Vec<String> {
    let mut board = Vec::new();

    for (i, row) in minefield.into_iter().enumerate() {
        let mut res_str = String::new();

        for (j, col) in row.char_indices() {
            let mut mine_count = 0;

            if col == ' ' {
                if i > 0 {
                    if minefield[i - 1].chars().nth(j).unwrap() == '*' {
                        mine_count += 1;
                    }
                }
                if i < minefield.len() - 1 {
                    if minefield[i + 1].chars().nth(j).unwrap() == '*' {
                        mine_count += 1;
                    }
                }
                if j > 0 {
                    if minefield[i].chars().nth(j - 1).unwrap() == '*' {
                        mine_count += 1;
                    }
                }
                if j < row.len() - 1 {
                    if minefield[i].chars().nth(j + 1).unwrap() == '*' {
                        mine_count += 1;
                    }
                }
                if i > 0 && j > 0 {
                    if minefield[i -1].chars().nth(j - 1).unwrap() == '*' {
                        mine_count += 1;
                    }
                }
                if i > 0 && j < row.len() - 1 {
                    if minefield[i - 1].chars().nth(j + 1).unwrap() == '*' {
                        mine_count += 1;
                    }
                }
                if i < minefield.len() - 1 && j > 0 {
                    if minefield[i + 1].chars().nth(j - 1).unwrap() == '*' {
                        mine_count += 1;
                    }
                }
                if i < minefield.len() - 1 && j < row.len() - 1 {
                    if minefield[i + 1].chars().nth(j + 1).unwrap() == '*' {
                        mine_count += 1;
                    }
                }

                if mine_count > 0 {
                    res_str.push_str(mine_count.to_string().as_str());
                } else {
                    res_str.push(' ');
                }
            } else {
                res_str.push(col);
            }
        }

        board.push(res_str);
    }

    board
}