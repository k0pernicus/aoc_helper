use std::collections::BTreeMap;

pub fn map_char_occurences<'a>(s: &'a str, count_whitespaces: bool) -> BTreeMap<char, u32> {
    let mut s_cpy = s;
    if !count_whitespaces {
        s_cpy = s.trim();
    }
    let mut count: BTreeMap<char, u32> = BTreeMap::new();
    for c in s_cpy.chars() {
        *count.entry(c).or_insert(0) += 1;
    }
    return count;
}

pub fn map_char_indexes<'a>(s: &'a str, index_whitespaces: bool) -> BTreeMap<char, Vec<usize>> {
    let mut s_cpy = s;
    if !index_whitespaces {
        s_cpy = s.trim();
    }
    let mut indexes: BTreeMap<char, Vec<usize>> = BTreeMap::new();
    for (index, c) in s_cpy.chars().enumerate() {
        indexes.entry(c).or_insert(Vec::new()).push(index);
    }
    return indexes;
}
