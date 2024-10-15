#[macro_export]
macro_rules! hash_map {
    () => {
        std::collections::HashMap::new()
    };

    ($($key:expr => $val:expr),+ $(,)?) => {
        {
            let mut tmp_map = std::collections::HashMap::new();

            $(
                tmp_map.insert($key, $val);
            )+

            tmp_map
        }
    };
}
