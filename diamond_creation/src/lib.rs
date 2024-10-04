pub fn get_diamond(c: char) -> Vec<String> {
    let mut fig = Vec::new();
    let space = c as u8 - b'A' + 1;

    for i in 0..space {
        let curr_char = (b'A' + i) as char;

        let line = if curr_char == 'A' {
            format!("{:^width$}", curr_char, width = space as usize * 2 - 1)
        } else {
            format!(
                "{:ext$}{}{:int$}{}{:ext$}",
                "",
                curr_char,
                "",
                curr_char,
                "",
                ext = (space - (i + 1)) as usize,
                int = ((i - 1) * 2 + 1) as usize
            )
        };

        fig.push(line);
    }

    for i in (0..fig.len() - 1).rev() {
        fig.push(fig[i].clone())
    }

    fig
}
