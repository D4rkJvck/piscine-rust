pub fn markdown_to_html(s: &str) -> String {
    let mut html = String::new();

    let lines = s.lines();

    for line in lines {
        if line.is_empty() {
            html.push('\n');
            continue;
        }

        html.push_str(&get_leading_whitespace(line));

        let words: Vec<&str> = line.split_whitespace().collect();

        if words.len() < 2 {
            html.push_str(&words[0]);
            html.push('\n');
            continue;
        }

        // Convert Headings
        match words[0] {
            "#" => html.push_str(&heading(1, &words[1..].join(" "))),
            "##" => html.push_str(&heading(2, &words[1..].join(" "))),
            "###" => html.push_str(&heading(3, &words[1..].join(" "))),
            ">" => html.push_str(
                format!(
                    "<{}>{}</{}>\n",
                    &words[1].to_lowercase(),
                    &words[1..].join(" "),
                    &words[1].to_lowercase()
                )
                .as_str(),
            ),
            _ => html.push_str(format!("{}\n", &words.join(" ")).as_str()),
        }
    }

    html = html
        .split("**")
        .collect::<Vec<&str>>()
        .join("<strong>")
        .split("*")
        .collect::<Vec<&str>>()
        .join("<em>");

    html
}

//----------------------------------------------------------------

fn get_leading_whitespace(s: &str) -> String {
    s.chars().take_while(|c| c.is_whitespace()).collect()
}

fn heading(i: usize, text: &str) -> String {
    format!("<h{}>{}</h{}>\n", i, text, i)
}
