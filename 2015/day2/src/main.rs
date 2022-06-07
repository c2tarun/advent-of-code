use std::cmp;
use std::fs;
fn main() {
    println!("Hello, world!");
    let content = fs::read_to_string("input.txt").unwrap();

    let mut total_area = 0;
    let mut total_ribbon_length: i32 = 0;

    for line in content.lines() {
        let mut elements: Vec<i32> = line.split('x').map(|v| v.parse::<i32>().unwrap()).collect();
        elements.sort();
        let area = required_paper_area(&elements);
        total_area += area;
        total_ribbon_length += required_ribbon_size(&elements);
    }

    println!("Total area required: {}", total_area);
    println!("Total ribbon length: {}", total_ribbon_length);
}

fn required_paper_area(elements: &Vec<i32>) -> i32 {
    let surface1 = elements[0] * elements[1];
    let surface2 = elements[1] * elements[2];
    let surface3 = elements[2] * elements[0];

    let smallest_surface = elements[0];
    return 2 * (surface1 + surface2 + surface3) + smallest_surface;
}

fn required_ribbon_size(elements: &Vec<i32>) -> i32 {
    return elements[0] * 2 + elements[1] * 2 + elements[0] * elements[1] * elements[2];
}
