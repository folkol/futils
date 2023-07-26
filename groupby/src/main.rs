use std::collections::HashMap;
use std::io::stdin;

fn main() {
    // groupby field one
    // todo: custom field, custom separator, maybe regex?:w
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    for line in stdin().lines().map_while(Result::ok) {
        if line.is_empty() {
            continue;
        }
        let key = line.split_whitespace().next().unwrap();
        groups.entry(key.to_owned()).or_insert_with(Vec::new).push(line);
    }
    let mut entries: Vec<_> = groups.iter().collect();
    entries.sort_by_key(|(k, _)| *k);
    for (key, values) in entries {
        let mut values: Vec<_> = values.iter().collect();
        values.sort();
        for value in values {
            println!("{key}:{value}")
        }
    }
}
