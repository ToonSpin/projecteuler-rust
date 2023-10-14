use std::collections::HashSet;

fn check_passcode_against_answer(passcode: &[char], answer: &(char, char, char)) -> bool {
    for i in 0..passcode.len() {
        if passcode[i] != answer.0 {
            continue;
        }

        for j in i+1..passcode.len() {
            if passcode[j] != answer.1 {
                continue;
            }

            for k in j+1..passcode.len() {
                if passcode[k] == answer.2 {
                    return true;
                }
            }
        }
    }
    false
}

fn check_passcode(passcode: &[char], answers: &Vec<(char, char, char)>) -> bool {
    answers.iter().all(|answer| check_passcode_against_answer(passcode, answer))
}

fn check_permutations(elements: &[char], mut so_far: Vec<char>, answers: &Vec<(char, char, char)>, level: u32) -> Option<Vec<char>> {
    if elements.len() == 0 {
        if check_passcode(&so_far, answers) {
            return Some(so_far.clone());
        } else {
            return None;
        }
    }

    for i in 0..elements.len() {
        let mut new_elements: Vec<char> = Vec::from(&elements[..i]);
        new_elements.extend_from_slice(&elements[i+1..]);
        so_far.push(elements[i]);
        if let Some(v) = check_permutations(&new_elements, so_far.clone(), answers, level + 1) {
            return Some(v);
        }
        so_far.pop();
    }

    None
}

fn main() {
    let input = include_str!("problem079.txt");
    let mut answers: HashSet<(char, char, char)> = HashSet::new();
    let elements = vec!['0', '1', '2', '3', '6', '7', '8', '9'];
    for line in input.lines() {
        let line = line.as_bytes();
        answers.insert((
            line[0] as char,
            line[1] as char,
            line[2] as char,
        ));
    }
    let answers: Vec<(char, char, char)> = answers.into_iter().collect();
    let matching_code = check_permutations(&elements, Vec::new(), &answers, 0);
    let matching_code: String = matching_code.unwrap().into_iter().collect();
    println!("The smallest matching code is: {}", matching_code);
}
