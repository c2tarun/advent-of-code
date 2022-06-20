use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
enum Gate {
    AND,
    OR,
    NOT,
    LSHIFT,
    RSHIFT,
}

impl FromStr for Gate {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "AND" => Ok(Gate::AND),
            "OR" => Ok(Gate::OR),
            "NOT" => Ok(Gate::NOT),
            "LSHIFT" => Ok(Gate::LSHIFT),
            "RSHIFT" => Ok(Gate::RSHIFT),
            _ => Err(format!("Unknown gate found {}", input)),
        }
    }
}

#[derive(Debug)]
struct Circuit {
    op1: Option<String>,
    op2: Option<String>,
    gate: Gate,
}

impl FromStr for Circuit {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = input.trim().split(" ").collect();
        match splits.len() {
            2 => Ok(Circuit {
                op1: Some(String::from(splits[1])),
                op2: None,
                gate: Gate::from_str(splits[0]).unwrap(),
            }),
            3 => Ok(Circuit {
                op1: Some(String::from(splits[0])),
                op2: Some(String::from(splits[2])),
                gate: Gate::from_str(splits[1]).unwrap(),
            }),
            _ => Err(format!("Unknown input: {}", input)),
        }
    }
}

fn main() {
    let c_map = build_c_map("input.txt");
    println!("{:?}", c_map);
}

fn build_c_map(filename: &str) -> HashMap<String, Circuit> {
    let mut c_map: HashMap<String, Circuit> = HashMap::new();
    let contents = fs::read_to_string(&filename).unwrap();
    for line in contents.lines() {
        let splits: Vec<&str> = line.split("->").collect();
        c_map.insert(
            String::from(splits[1].trim()),
            Circuit::from_str(splits[0]).unwrap(),
        );
    }
    c_map
}

fn dfs(key: char, v_map: &mut HashMap<char, i32>, c_map: &HashMap<char, String>) -> i32 {
    if v_map.contains_key(&key) {
        return *(v_map.get(&key).unwrap());
    }
    unimplemented!();
}

fn split_circuit(circuit: &str) -> (&str, Gate, Option<&str>) {
    unimplemented!();
}
