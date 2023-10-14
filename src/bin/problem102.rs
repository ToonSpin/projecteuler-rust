type Point = (i64, i64);

#[derive(Debug)]
struct Triangle {
    p: Point,
    q: Point,
    r: Point,
}

impl Triangle {
    fn origin_parity(p: Point, q: Point) -> i64 {
        let p_prime = (q.0 - p.0, q.1 - p.1);
        let o_prime = (-p.0, -p.1);
        o_prime.1 * p_prime.0 - o_prime.0 * p_prime.1
    }

    fn contains_origin(&self) -> bool {
        let p1 = Self::origin_parity(self.p, self.q);
        let p2 = Self::origin_parity(self.q, self.r);
        let p3 = Self::origin_parity(self.r, self.p);

        if p1 < 0 && p2 < 0 && p3 < 0 {
            true
        } else if p1 > 0 && p2 > 0 && p3 > 0 {
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut input = Vec::new();
    for line in include_str!("problem102.txt").lines() {
        let parsed: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
        input.push(Triangle {
            p: (parsed[0], parsed[1]),
            q: (parsed[2], parsed[3]),
            r: (parsed[4], parsed[5]),
        });
    }
    let count = input.iter().filter(|t| t.contains_origin()).count();
    println!("The number of triangles that contain the origin: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_examples() {
        let example = Triangle {
            p: (-340, 495),
            q: (-153, -910),
            r: (835, -947),
        };
        assert!(example.contains_origin());

        let example = Triangle {
            p: (-175, 41),
            q: (-421, -714),
            r: (574, -645),
        };
        assert!(!example.contains_origin());
    }
}