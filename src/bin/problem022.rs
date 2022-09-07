fn main() {
    let input = include_str!("problem22.txt");

    let mut names: Vec<&str> = Vec::new();
    for raw_name in input.split(',') {
        names.push(&raw_name[1..raw_name.len() - 1]);
    }
    names.sort();

    let mut total: u32 = 0;
    for (i, name) in names.iter().enumerate() {
        let raw_score: u32 = name.as_bytes().iter().map(|c| (c - b'A' + 1) as u32).sum();
        total += (i as u32 + 1) * raw_score;
    }

    println!("The total of all the scores is: {}", total);
}
