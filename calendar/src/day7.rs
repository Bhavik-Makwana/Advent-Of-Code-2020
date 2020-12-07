use std::fmt::Error;
// use petgraph::graph::Graph;
// use petgraph::dot::Dot;
// use petgraph::visit::Dfs;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn part_one(bags: &Vec<String>) -> Result<i32, Error> {
    let adj_list = create_adj_list_reversed(&bags);
    let mut visited = HashSet::new();
    let x = dfs(&"shinygold", &mut visited, &adj_list, 1);
    Ok(x-1)
}

fn create_adj_list_reversed(bags: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();
    for line in bags.iter() {
        let mut a = line.split(" contain ");
        let key = a.next().unwrap().replace("bags", "")
        .replace(" ", "");
        let values = a.next().unwrap().split(", ");
        for (_, val) in values.enumerate() {
            let curr = val.replace("bags", "")
                        .replace("bag","")
                        .replace(" .", "");
            let mut curr = curr.split_whitespace();
            let count = curr.next().unwrap();
            let mut new_val = String::from("no");
            let mut c = 0;
            if count != "no".to_string() {
                new_val = String::from(curr.next().unwrap()); 
                new_val.push_str(curr.next().unwrap());
                c = count.parse::<i32>().unwrap(); 
            }
            adj_list.entry(new_val).or_insert(Vec::new()).push(key.clone());
        }
        
    }
    adj_list
}

fn dfs(v: &str, visited: &mut HashSet<String>, graph: &HashMap<String, Vec<String>>, mut count: i32) -> i32 {
    visited.insert(v.to_string());
    if graph.get(v) != None {    
    for neighbour in graph.get(v).unwrap().iter() {
        if !visited.contains(&neighbour[..]) {
            // println!("{}", neighbour);
            count = dfs(neighbour, visited, graph, count+1);
        }
    }
    }
    return count;
}

pub fn part_two(bags: &Vec<String>) -> Result<i32, Error> {
    let adj_list = create_adj_list(&bags);
    let mut total = 0;
    for neighbour in adj_list.get("shinygold").unwrap().iter() {
        let c= dfs_no_visited((&neighbour.0[..], neighbour.1), &adj_list);
        total  +=  neighbour.1 + neighbour.1 *c;
        // println!("{:?}", neighbour);
    }
    
    Ok(total)
}

fn dfs_no_visited(v: (&str, i32), graph: &HashMap<String, Vec<(String,i32)>>) -> i32 {
    
    let mut count  = 0;

    if graph.get(v.0) != None {    
        for neighbour in graph.get(v.0).unwrap().iter() {
    
            count +=  neighbour.1 + neighbour.1 *dfs_no_visited((&neighbour.0[..], neighbour.1), graph);
            
        }
    }
    return count;
}

fn create_adj_list(bags: &Vec<String>) -> HashMap<String, Vec<(String, i32)>> {
    let mut adj_list: HashMap<String, Vec<(String, i32)>> = HashMap::new();
    for line in bags.iter() {
        let mut a = line.split(" contain ");
        let key = a.next().unwrap().replace("bags", "")
        .replace(" ", "");
        let values = a.next().unwrap().split(", ");
        let mut nodes = Vec::new();
        for (_, val) in values.enumerate() {
            let curr = val.replace("bags", "")
                        .replace("bag","")
                        .replace(" .", "");
            let mut curr = curr.split_whitespace();
            let count = curr.next().unwrap();
            let mut new_val = String::from("no");
            let mut c = 0;
            if count != "no".to_string() {
                new_val = String::from(curr.next().unwrap()); 
                new_val.push_str(curr.next().unwrap());
                c = count.parse::<i32>().unwrap(); 
            }
            nodes.push((new_val, c));
        }
        adj_list.insert(key, nodes);
    }
    adj_list
}