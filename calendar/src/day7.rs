use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Error;

#[derive(Debug, Clone)]
struct GraphNode {
    name: String,
    weight: i32,
}

#[derive(Debug, Clone)]
struct Graph {
    graph: HashMap<String, Vec<GraphNode>>,
}

pub fn part_one(bags: &Vec<String>) -> Result<i32, Error> {
    let adj_list = create_adj_list_reversed(&bags);
    let mut visited = HashSet::new();
    let x = dfs(&"shinygold", &mut visited, &adj_list, 1);
    Ok(x - 1)
}

fn create_adj_list_reversed(bags: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();
    for line in bags.iter() {
        let mut a = line.split(" contain ");
        let key = a.next().unwrap().replace("bags", "").replace(" ", "");

        let values = a.next().unwrap().split(", ");
        for (_, val) in values.enumerate() {
            let curr = val.replace("bags", "").replace("bag", "").replace(" .", "");
            let mut curr = curr.split_whitespace();
            let count = curr.next().unwrap();
            let mut new_val = String::from("no");
            if count != "no".to_string() {
                new_val = String::from(curr.next().unwrap());
                new_val.push_str(curr.next().unwrap());
            }
            adj_list
                .entry(new_val)
                .or_insert(Vec::new())
                .push(key.clone());
        }
    }
    adj_list
}

fn dfs(
    v: &str,
    visited: &mut HashSet<String>,
    graph: &HashMap<String, Vec<String>>,
    mut count: i32,
) -> i32 {
    visited.insert(v.to_string());
    if graph.get(v) != None {
        for neighbour in graph.get(v).unwrap().iter() {
            if !visited.contains(&neighbour[..]) {
                count = dfs(neighbour, visited, graph, count + 1);
            }
        }
    }
    return count;
}

pub fn part_two(bags: &Vec<String>) -> Result<i32, Error> {
    let graph = create_adj_list(&bags);
    let mut total = 0;
    for neighbour in graph.graph.get("shinygold").unwrap().iter() {
        let c = dfs_no_visited(neighbour.clone(), &graph);
        total += neighbour.weight + neighbour.weight * c;
    }

    Ok(total)
}

fn dfs_no_visited(v: GraphNode, graph: &Graph) -> i32 {
    let mut count = 0;

    match graph.graph.get(&v.name) {
        Some(g) => {
            for neigh in g.iter() {
                count += neigh.weight + neigh.weight * dfs_no_visited(neigh.clone(), graph);
            }
        }
        None => (),
    }
    return count;
}

fn create_adj_list(bags: &Vec<String>) -> Graph {
    let mut adj_list: HashMap<String, Vec<GraphNode>> = HashMap::new();
    for line in bags.iter() {
        let mut a = line.split(" contain ");
        let key = a.next().unwrap().replace("bags", "").replace(" ", "");
        let values = a.next().unwrap().split(", ");
        let mut nodes = Vec::new();
        for (_, val) in values.enumerate() {
            let curr = val.replace("bags", "").replace("bag", "").replace(" .", "");
            let mut curr = curr.split_whitespace();
            let count = curr.next().unwrap();
            let mut new_val = String::from("no");
            let mut c = 0;
            if count != "no".to_string() {
                new_val = String::from(curr.next().unwrap());
                new_val.push_str(curr.next().unwrap());
                c = count.parse::<i32>().unwrap();
            }
            nodes.push(GraphNode {
                name: new_val,
                weight: c,
            });
        }
        adj_list.insert(key, nodes);
    }
    Graph { graph: adj_list }
}


#[cfg(test)]
mod tests {
    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day7.txt"));
        assert_eq!(crate::day7::part_one(&vec), Ok(4));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day7.txt"));
        assert_eq!(crate::day7::part_two(&vec), Ok(32));
    }
}