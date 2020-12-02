use std::collections::BTreeMap;

pub fn map_char_occurences<'a>(s: &'a str) -> BTreeMap<char, u32> {
    let mut count: BTreeMap<char, u32> = BTreeMap::new();
    for c in s.chars() {
        *count.entry(c).or_insert(0) += 1;
    }
    return count;
}

pub fn map_char_indexes<'a>(s: &'a str) -> BTreeMap<char, Vec<usize>> {
    let mut indexes: BTreeMap<char, Vec<usize>> = BTreeMap::new();
    for (index, c) in s.chars().enumerate() {
        indexes.entry(c).or_insert(Vec::new()).push(index);
    }
    return indexes;
}
