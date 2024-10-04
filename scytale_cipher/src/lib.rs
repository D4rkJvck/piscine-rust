pub fn scytale_cipher(msg: String, size: u32) -> String {
    let chunks = chunk_msg(&mut msg.clone(), size);

    let mut msg = String::new();

    for i in 0..size as usize {
        for chunk in chunks.iter() {
            if i < chunk.len() {
                msg.push(chunk[i]);
            } else {
                msg.push(' ');
            }
        }
    }

    msg.trim().to_string()
}

//________________________________________________________________
//

fn chunk_msg(msg: &mut String, size: u32) -> Vec<Vec<char>> {
    let mut sup: Vec<Vec<char>> = Vec::new();
    let mut sub: Vec<char> = Vec::new();

    for c in msg.chars() {
        sub.push(c);

        if sub.len() == size as usize {
            sup.push(sub.clone());
            sub.clear();
        }
    }

    if !sub.is_empty() {
        sup.push(sub);
    }

    sup
}
