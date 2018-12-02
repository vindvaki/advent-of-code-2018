use std::collections::HashMap;

pub fn count_by_value<'a, T: 'a, I>(data: I) -> HashMap<T, u32>
where
    I: Iterator<Item = T>,
    T: Eq + std::hash::Hash,
{
    let mut result = HashMap::new();
    for value in data {
        let count: u32 = *result.get(&value).unwrap_or(&0);
        result.insert(value, count + 1);
    }
    return result;
}

