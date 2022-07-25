use projecteuler::util::maxpathsum::max_path;
use projecteuler::util::maxpathsum::input_to_triangle;

fn main() {
    let triangle = input_to_triangle(include_str!("problem18.txt"));
    println!("The maximum path is: {}", max_path(triangle));
}
