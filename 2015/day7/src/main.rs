use std::collections::HashMap;
use std::fs;

fn main() {
    let c_map = build_c_map("input.txt");
    println!("{:?}", c_map);
}

fn build_c_map(filename: &str) -> HashMap<String, String> {
    let mut c_map: HashMap<String, String> = HashMap::new();
    let contents = fs::read_to_string(&filename).unwrap();
    for line in contents.lines() {
        let splits: Vec<&str> = line.split("->").collect();
        c_map.insert(String::from(splits[1].trim()), String::from(splits[0].trim()));
    }
    c_map
}

fn dfs(key: char, v_map: &mut HashMap<char, i32>, c_map: &HashMap<char, String>) -> i32 {
    if v_map.contains_key(&key) {
        return *(v_map.get(&key).unwrap());
    }
    unimplemented!();
}
