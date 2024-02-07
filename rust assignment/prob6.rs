fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return "".to_string();
    }

    let mut prefix = String::from("");
    for i in 0..strs[0].chars().count() {
        let c = strs[0].chars().nth(i).unwrap();
        for s in strs.iter() {
            if s.chars().nth(i) != Some(c) {
                return prefix;
            }
        }
        prefix.push(c);
    }

    prefix
}