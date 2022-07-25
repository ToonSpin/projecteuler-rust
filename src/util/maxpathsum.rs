pub fn input_to_triangle(input: &str) -> Vec<Vec<u32>> {
    let mut triangle = Vec::new();
    for line in input.lines() {
        let substrings = line.split(" ");
        triangle.push(substrings.map(|s| s.parse().unwrap()).collect());
    }
    triangle
}

fn process_line(previous: &Vec<u32>, next: &Vec<u32>) -> Vec<u32> {
    let mut result = Vec::new();
    let l = next.len();
    result.push(next[0] + previous[0]);
    for i in 1..=l-2 {
        let a = previous[i - 1];
        let b = previous[i];
        result.push(next[i] + if a > b { a } else { b })
    }
    result.push(next.last().unwrap() + previous.last().unwrap());
    result
}

pub fn max_path(triangle: Vec<Vec<u32>>) -> u32 {
    let result = triangle[1..].iter().fold(vec![triangle[0][0]], |a, e| process_line(&a, e));
    *result.iter().max().unwrap()
}
