use std::collections::{BinaryHeap, HashSet};

#[derive(Hash)]
struct Edge {
    from: usize,
    to: usize,
    cost: u32,
}

impl Eq for Edge {
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

fn main() {
    let mut input = Vec::new();
    let mut total_cost = 0;
    for line in include_str!("problem107.txt").lines() {
        let mut edges: Vec<(usize, u32)> = Vec::new();
        for (node, cost) in line.split(',').enumerate() {
            if cost != "-" {
                let (n, c) = (node, cost.parse().unwrap());
                total_cost += c;
                edges.push((n, c));
            }
        }
        input.push(edges);
    }
    total_cost /= 2;

    let mut tree = HashSet::new();
    let mut edges = HashSet::new();
    let mut queue = BinaryHeap::new();
    let starting_node = 0;
    tree.insert(starting_node);
    for (node, cost) in input[starting_node].iter() {
        queue.push(Edge {
            from: starting_node,
            to: *node,
            cost: *cost
        });
    }
    
    let mut min_cost = 0;
    loop {
        let new_edge = queue.pop();
        if let None = new_edge {
            break;
        }
        let new_edge = new_edge.unwrap();
        if tree.contains(&new_edge.to) {
            continue;
        }
        tree.insert(new_edge.to);
        min_cost += new_edge.cost;
        for (node, cost) in input[new_edge.to].iter() {
            if !tree.contains(node) {
                queue.push(Edge {
                    from: new_edge.to,
                    to: *node,
                    cost: *cost
                });
            }
        }
        edges.insert(new_edge);
    }

    println!("The maximum savings are: {}", total_cost - min_cost);
}
