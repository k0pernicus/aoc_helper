use std::collections::BTreeMap;

pub fn map_char_occurences<'a>(s: &'a str, count_whitespaces: bool) -> BTreeMap<char, u32> {
    let s_cpy = if count_whitespaces { s } else { s.trim() };
    let mut count: BTreeMap<char, u32> = BTreeMap::new();
    s_cpy
        .chars()
        .for_each(
            |c| *count.entry(c).or_insert(0) += 1
        );
    count
}

pub fn map_char_indexes<'a>(s: &'a str, index_whitespaces: bool) -> BTreeMap<char, Vec<usize>> {
    let s_cpy = if index_whitespaces { s } else { s.trim() };
    let mut indexes: BTreeMap<char, Vec<usize>> = BTreeMap::new();
    s_cpy
        .chars()
        .enumerate()
        .for_each(
            |(i, c)| indexes.entry(c).or_insert(Vec::new()).push(i)
        );
    indexes
}
