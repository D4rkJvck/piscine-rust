pub enum Security {
    Unknown,
    High,
    Medium,
    Low,
    BlockServer,
}

pub fn fetch_data(server: Result<String, String>, sec_lvl: Security) -> String {
    use self::Security::*;

    match server {
        Ok(url) => match sec_lvl {
            BlockServer => panic!("{}", url.to_string()),
            _ => url.to_string(),
        },
        Err(url) => match sec_lvl {
            Unknown => panic!(),
            High => panic!("ERROR: program stops"),
            Medium => String::from("WARNING: check the server"),
            Low => format!("Not found: {}", url),
            _ => unreachable!(),
        },
    }
}
