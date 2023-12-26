use std::collections::HashSet;

#[derive(Debug, Default)]
struct PathTree {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>, // undirected (node1_idx, node2_idx)
}

impl From<&str> for PathTree {
    fn from(value: &str) -> Self {
        let mut pt = PathTree::default();

        for l in value.trim().lines() {
            let (n1, nlist) = l.split_once(": ").unwrap();
            for n2 in nlist.trim().split_whitespace() {
                pt.add(n1.to_string(), n2.to_string());
            }
        }

        pt
    }
}

impl PathTree {
    fn add(&mut self, source_node: String, destination_node: String) {
        let source_idx = if !self.nodes.contains(&source_node) {
            let idx = self.nodes.len();
            self.nodes.push(source_node);
            idx
        } else {
            self.nodes.iter().position(|n| *n == source_node).unwrap()
        };

        let destination_idx = if !self.nodes.contains(&destination_node) {
            let idx = self.nodes.len();
            self.nodes.push(destination_node);
            idx
        } else {
            self.nodes
                .iter()
                .position(|n| *n == destination_node)
                .unwrap()
        };

        if !self.edges.contains(&(source_idx, destination_idx)) {
            self.edges.push((source_idx, destination_idx));
        }
    }

    fn delete_edge(&mut self, node1: String, node2: String) {
        let node1_idx = self.nodes.iter().position(|n| *n == node1).unwrap();
        let node2_idx = self.nodes.iter().position(|n| *n == node2).unwrap();
        if let Some(edge_idx) = self.edges.iter().position(|n| *n == (node1_idx, node2_idx)) {
            self.edges.swap_remove(edge_idx);
        }
        if let Some(edge_idx) = self.edges.iter().position(|n| *n == (node2_idx, node1_idx)) {
            self.edges.swap_remove(edge_idx);
        }
    }

    fn count_nodes_in_component(&self, starting_node: String) -> usize {
        let starting_node_idx = self.nodes.iter().position(|n| *n == starting_node).unwrap();
        let mut queue = vec![starting_node_idx];
        let mut visited = HashSet::new();

        while let Some(current_node_idx) = queue.pop() {
            if visited.contains(&current_node_idx) {
                continue;
            }

            visited.insert(current_node_idx);

            for n in self
                .edges
                .iter()
                .filter(|(n, _)| *n == current_node_idx)
                .map(|(_, n)| n)
            {
                if !visited.contains(n) {
                    queue.push(*n);
                }
            }
            for n in self
                .edges
                .iter()
                .filter(|(_, n)| *n == current_node_idx)
                .map(|(n, _)| n)
            {
                if !visited.contains(n) {
                    queue.push(*n);
                }
            }
        }

        visited.len()
    }

    fn _print_graphviz(&self) {
        println!("graph AOC {{");
        for (src, dst) in &self.edges {
            let src_lbl = self.nodes.get(*src).unwrap();
            let dst_lbl = self.nodes.get(*dst).unwrap();
            println!("{} -- {}", src_lbl, dst_lbl);
        }
        println!("}}");
    }
}

pub fn part_one(_input: &str) -> Option<usize> {
    let mut pathtree = PathTree::from(_input);

    // To solve this, uncomment the following line and use the output
    // to view the graph visually: `dot -Tsvg -Kneato src/visualizations/25_01_example.dot > src/visualizations/25_01_example.svg`
    //pathtree._print_graphviz();

    // Then note down the three edges that combine the two components of the graph
    pathtree.delete_edge("zhb".to_string(), "vxr".to_string());
    pathtree.delete_edge("jbx".to_string(), "sml".to_string());
    pathtree.delete_edge("szh".to_string(), "vqj".to_string());

    Some(
        pathtree.count_nodes_in_component("zhb".to_string())
            * pathtree.count_nodes_in_component("vxr".to_string()),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some(54));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
