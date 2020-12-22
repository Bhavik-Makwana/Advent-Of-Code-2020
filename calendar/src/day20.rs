use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Error;
use std::hash::{Hash, Hasher};
// use itertools::Itertools; // 0.8.2
use std::cmp::Eq;

#[derive(Debug, Default, Clone)]
struct Tile {
    id: i64,
    image: Vec<Vec<char>>,
}

#[derive(Debug, Default, Clone, Eq)]
struct Node {
    id: i64,
    image: Vec<Vec<char>>,
    orientation: i32,
    //     top: Vec<char>,
    //     bottom: Vec<char>,
    //     left: Vec<char>,
    //     right: Vec<char>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        // self.image.hash(state);
    }
}

#[derive(Debug, Clone)]
struct Graph {
    graph: HashMap<Node, Vec<Node>>,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (key, value) in self.graph.iter() {
            write!(f, "  node: {}\n    adj list: ", key)?;
            for i in value.iter() {
                write!(f, "{}, ", i)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn part_one(input: &Vec<Vec<String>>) -> Result<i64, Error> {
    // let all_edges = HashSet::new();
    let re = Regex::new(r"[^0-9.]").unwrap();
    let mut tiles = Vec::new();
    let mut just_tiles = Vec::new();
    for tile in input.iter() {
        let mut curr: Tile = Tile {
            id: 0,
            image: Vec::<Vec<char>>::new(),
        };
        let mut c = Vec::new();
        for (i, line) in tile.iter().enumerate() {
            if i == 0 {
                let x = re.replace_all(line, "");
                curr.id = x.parse::<i64>().unwrap();
            } else {
                curr.image.push(line.chars().collect::<Vec<char>>());
                c.push(line.chars().collect::<Vec<char>>());
            }
        }
        tiles.push(curr);
        just_tiles.push(c);
    }

    let mut prod: i64 = 1;
    for i in 0..tiles.len() {
        let mut count = 0;
        for j in 0..tiles.len() {
            if i != j {
                count += compare_edges(&mut tiles, i, j)
            }
        }
        // println!("count {}", count);
        if count == 2 {
            // println!("{}", tiles[i].id);
            prod *= tiles[i].id;
        }
    }
    // for perm in just_tiles.iter().permutations(just_tiles.len()).unique() {
    //     for row in perm.iter() {
    //         println!("{:?}", row);
    //     }
    // }
    Ok(prod) // 156
}

fn compare_edges(tiles: &mut Vec<Tile>, i: usize, j: usize) -> i32 {
    // println!("Tile 1: {}", tiles[i].id);

    let mut count = 0;
    for _ in 0..4 {
        rotate_90_cw(&mut tiles[j]);

        let mut matching = false;
        for _ in 0..4 {
            rotate_90_cw(&mut tiles[i]);
            let mut c_matching = true;
            for k in 0..tiles[i].image[0].len() {
                if tiles[j].image[0][k] != tiles[i].image[0][k] {
                    c_matching = false;
                }
            }

            if c_matching {
                matching = true;
            }
            c_matching = true;

            flip_matrix(&mut tiles[i], false);
            for k in 0..tiles[i].image[0].len() {
                if tiles[j].image[0][k] != tiles[i].image[0][k] {
                    c_matching = false;
                }
            }
            flip_matrix(&mut tiles[i], false);

            if c_matching {
                matching = true;
            }
            c_matching = true;

            flip_matrix(&mut tiles[i], true);
            for k in 0..tiles[i].image[0].len() {
                if tiles[j].image[0][k] != tiles[i].image[0][k] {
                    c_matching = false;
                }
            }
            flip_matrix(&mut tiles[i], true);

            if c_matching {
                matching = true;
            }
            c_matching = true;

            flip_matrix(&mut tiles[i], false);
            flip_matrix(&mut tiles[i], true);
            for k in 0..tiles[i].image[0].len() {
                if tiles[j].image[0][k] != tiles[i].image[0][k] {
                    c_matching = false;
                }
            }
            flip_matrix(&mut tiles[i], true);
            flip_matrix(&mut tiles[i], false);

            if c_matching {
                matching = true;
            }
        }
        if matching {
            // println!("{}", count);
            count += 1;
        }
    }
    // println!("ret {}", count);
    count
}

fn rotate_90_cw(tile: &mut Tile) {
    let n = 10;
    for i in 0..n / 2 {
        for j in i..n - i - 1 {
            let temp = tile.image[i][j];
            tile.image[i][j] = tile.image[j][n - 1 - i];
            tile.image[j][n - 1 - i] = tile.image[n - 1 - i][n - 1 - j];
            tile.image[n - 1 - i][n - 1 - j] = tile.image[n - 1 - j][i];
            tile.image[n - 1 - j][i] = temp;
        }
    }
}

fn flip_matrix(tile: &mut Tile, h: bool) {
    if h {
        for i in 0..tile.image.len() {
            let mut x = Vec::new();
            for j in (0..tile.image[0].len()).rev() {
                // println!("{}", tile.image[i][j]);
                x.push(tile.image[i][j]);
            }
            // println!("{:?}", x);
            for j in 0..tile.image.len() {
                tile.image[i][j] = x[j];
            }
        }
    } else {
        // for row in tile.image.iter() {
        //     println!("{:?}", row);
        // }
        // println!("");
        for i in 0..tile.image[0].len() {
            let mut x = Vec::new();
            for j in (0..tile.image.len()).rev() {
                x.push(tile.image[j][i]);
            }
            for j in 0..tile.image.len() {
                tile.image[j][i] = x[j];
            }
        }
        // for row in tile.image.iter() {
        //     println!("{:?}", row);
        // }
    }
}

fn rotate_90_cw_p2(tile: &mut Node) {
    let n = 10;
    for i in 0..n / 2 {
        for j in i..n - i - 1 {
            let temp = tile.image[i][j];
            tile.image[i][j] = tile.image[j][n - 1 - i];
            tile.image[j][n - 1 - i] = tile.image[n - 1 - i][n - 1 - j];
            tile.image[n - 1 - i][n - 1 - j] = tile.image[n - 1 - j][i];
            tile.image[n - 1 - j][i] = temp;
        }
    }
    tile.orientation = (tile.orientation + 1) % 4;
}

fn flip_matrix_p2(tile: &mut Node, h: bool) {
    if h {
        for i in 0..tile.image.len() {
            let mut x = Vec::new();
            for j in (0..tile.image[0].len()).rev() {
                // println!("{}", tile.image[i][j]);
                x.push(tile.image[i][j]);
            }
            // println!("{:?}", x);
            for j in 0..tile.image.len() {
                tile.image[i][j] = x[j];
            }
        }
    } else {
        // for row in tile.image.iter() {
        //     println!("{:?}", row);
        // }
        // println!("");
        for i in 0..tile.image[0].len() {
            let mut x = Vec::new();
            for j in (0..tile.image.len()).rev() {
                x.push(tile.image[j][i]);
            }
            for j in 0..tile.image.len() {
                tile.image[j][i] = x[j];
            }
        }
        // for row in tile.image.iter() {
        //     println!("{:?}", row);
        // }
    }
}

fn lineup(tiles: &mut Vec<Node>, i: usize, j: usize) -> bool {
    // println!("Tile 1: {}", tiles[i].id);

    for _ in 0..4 {
        // for tile in tiles.iter() {
        // println!("{:?}", graph.graph.get(&tiles[j]));
        // }
        // println!("");
        rotate_90_cw_p2(&mut tiles[j]);
        // for tile in tiles.iter() {
        // println!("{:?}", graph.graph.get(&tiles[j]));
        // }
        let mut matching = false;
        for t in 0..4 {
            // println!("t {}", t);

            let mut c_matching = true;

            rotate_90_cw_p2(&mut tiles[i]);

            for k in 0..tiles[i].image[0].len() {
                if tiles[j].image[0][k] != tiles[i].image[0][k] {
                    c_matching = false;
                }
            }

            if c_matching {
                matching = true;
            }
            c_matching = true;

            flip_matrix_p2(&mut tiles[i], false);
            for k in 0..tiles[i].image[0].len() {
                if tiles[j].image[0][k] != tiles[i].image[0][k] {
                    c_matching = false;
                }
            }
            flip_matrix_p2(&mut tiles[i], false);

            if c_matching {
                matching = true;
            }
            c_matching = true;

            flip_matrix_p2(&mut tiles[i], true);
            for k in 0..tiles[i].image[0].len() {
                if tiles[j].image[0][k] != tiles[i].image[0][k] {
                    c_matching = false;
                }
            }
            flip_matrix_p2(&mut tiles[i], true);

            if c_matching {
                matching = true;
            }
            c_matching = true;

            flip_matrix_p2(&mut tiles[i], false);
            flip_matrix_p2(&mut tiles[i], true);
            for k in 0..tiles[i].image[0].len() {
                if tiles[j].image[0][k] != tiles[i].image[0][k] {
                    c_matching = false;
                }
            }
            flip_matrix_p2(&mut tiles[i], true);
            flip_matrix_p2(&mut tiles[i], false);

            if c_matching {
                matching = true;
            }
        }
        if matching {
            // println!("{}", count);

            // println!("mi {}", tiles[i]);
            return true;
        }
    }
    false
}

fn check_right(tile1: &mut Node, tile2: &mut Node) -> bool {
    // println!("Tile 1: {}   Tile 2: {}", tile1, tile2);

    
    let mut matching = false;
    // for _ in 0..4 {
    //     rotate_90_cw_p2(tile1);
    for _ in 0..4 {
        // println!("t {}", t);

        let mut c_matching = true;

        rotate_90_cw_p2( tile2);

        for k in 0..tile1.image[0].len() {
            if tile2.image[k][0] != tile1.image[k][9] {
                c_matching = false;
            }
        }

        if c_matching {
            return true;
        }

        flip_matrix_p2( tile2, false);
        for k in 0..tile1.image[0].len() {
            if tile2.image[k][0] != tile1.image[k][9] {
                return false;
            }
        }
        
        if c_matching {
            matching = true;
        }
        flip_matrix_p2( tile2, false);

        flip_matrix_p2( tile2, true);
        for k in 0..tile1.image[0].len() {
            if tile2.image[k][0] != tile1.image[k][9] {
                return  false;
            }
        }
        
        if c_matching {
            return true;
        }
        flip_matrix_p2( tile2, true);

        flip_matrix_p2( tile2, false);
        flip_matrix_p2( tile2, true);
        for k in 0..tile1.image[0].len() {
            if tile2.image[k][0] != tile1.image[k][9] {
                c_matching = false;
            }
        }
        
        if c_matching {
            return true;
        }
        flip_matrix_p2( tile2, true);
        flip_matrix_p2( tile2, false);
    }


    false
}
fn check_bottom(tile1: &mut  Node, tile2: &mut Node) -> bool {
    // println!("Tile 1: {}", tiles[i].id);


    let mut matching = false;
    // for _ in 0..4 {        
    //     rotate_90_cw_p2(tile1);

    for _ in 0..4 {
        // println!("t {}", t);

        let mut c_matching = true;

        rotate_90_cw_p2( tile2);

        for k in 0..tile1.image[0].len() {
            if tile2.image[0][k] != tile1.image[9][k] {
                c_matching = false;
            }
        }
       
        if c_matching {
            return true;
        }
        // c_matching = true;

        flip_matrix_p2( tile2, false);
        for k in 0..tile1.image[0].len() {
            if tile2.image[0][k] != tile1.image[9][k] {
                c_matching = false;
            }
        }
     
        
        if c_matching {
            return true;
        }
        flip_matrix_p2( tile2, false);
        // c_matching = true;

        flip_matrix_p2( tile2, true);
        for k in 0..tile1.image[0].len() {
            if tile2.image[0][k] != tile1.image[9][k] {
                c_matching = false;
            }
        }
  
        
        if c_matching {
            return  true;
        }
        flip_matrix_p2( tile2, true);
        // c_matching = true;

        flip_matrix_p2( tile2, false);
        flip_matrix_p2( tile2, true);
        for k in 0..tile1.image[0].len() {
            if tile2.image[0][k] != tile1.image[9][k] {
                c_matching = false;
            }
        }
        
        
        if c_matching {
            return true;
        }
        flip_matrix_p2( tile2, true);
        flip_matrix_p2( tile2, false);
    }
   
    // }
    // println!("i{}", tiles[i]);
    // println!("j{}", tiles[j]);
    // println!("ret {}", count);
    false
}
fn dfs(v: &Node, visited: &mut HashSet<Node>, graph: &mut Graph, tiles: &mut Vec<Node>) {
    visited.insert(v.clone());

    let mut i = 0;
    for (x, p) in tiles.iter().enumerate() {
        if p == v {
            i = x;
            break;
        }
    }
    // for tile in tiles.iter() {
    //     println!("{:?}", graph.graph.get_mut(&tile));
    // }
    // println!("");
    // println!("v {} i {}",v, i);

    for j in 0..tiles.len() {
        // println!("j {}", j);
        if i != j {
            // for tile in tiles.iter() {
            //     println!("{:?}", graph.graph.get(&tile));
            // }
            if lineup(tiles, i, j) {
                // println!("back i {} ", tiles[i]);
                // println!("back j {} ", tiles[j]);
                // println!("graph:\n{}", graph);
                // for tile in tiles.iter() {
                //     println!("{:?}", graph.graph.get(&tile));
                // }
                graph.graph.get_mut(&v).unwrap().push(tiles[j].clone());
            }
            if !visited.contains(&tiles[j]) {
                dfs(&tiles[j].clone(), visited, graph, tiles);
            }
        }
    }

    // for neighbour in graph.get(v).unwrap().iter() {
    // if !visited.contains(&neighbour[..]) {
    //     count = dfs(neighbour, visited, graph, count + 1);
    // }
    // }
}

pub fn part_two(input: &Vec<Vec<String>>) -> Result<i32, Error> {
    // let all_edges = HashSet::new();
    let re = Regex::new(r"[^0-9.]").unwrap();
    let mut tiles = Vec::new();
    let m: HashMap<Node, Vec<Node>> = HashMap::new();
    let mut graph: Graph = Graph { graph: m };
    for tile in input.iter() {
        let mut curr: Node = Node {
            id: 0,
            image: Vec::<Vec<char>>::new(),
            orientation: 0,
        };
        for (i, line) in tile.iter().enumerate() {
            if i == 0 {
                let x = re.replace_all(line, "");
                curr.id = x.parse::<i64>().unwrap();
            } else {
                curr.image.push(line.chars().collect::<Vec<char>>());
            }
        }
        tiles.push(curr.clone());
        graph.graph.insert(curr, Vec::new());
    }

    let mut visited = HashSet::new();
    // for (node, value) in graph.iter() {
    //     println!("{} {:?}", node, value);
    // }
    // println!("{}", graph);

    let _x = dfs(&tiles[0].clone(), &mut visited, &mut graph, &mut tiles);
    println!("{}", graph);
    let mut visited: HashSet<Node> = HashSet::new();
    let mut start_node = tiles[0].clone();
    for (key, value) in graph.graph.iter() {
        if value.len() == 2 {
            start_node = key.clone();
        }
    }
    rotate_90_cw_p2(&mut start_node);
    rotate_90_cw_p2(&mut start_node);
    rotate_90_cw_p2(&mut start_node);
    // flip_matrix_p2(&mut start_node, false);
    // rotate_90_cw_p2(&mut start_node);
    // rotate_90_cw_p2(&mut start_node);
    // rotate_90_cw_p2(&mut start_node);
    // rotate_90_cw_p2(&mut start_node);
    // rotate_90_cw_p2(&mut start_node);
    // rotate_90_cw_p2(&mut start_node);
    println!("start node {}", start_node);
    let mut grid: Vec<Vec<Node>> = vec![vec![Default::default(); 4]; 3];
    visited.insert(start_node.clone());
    grid[0][0] = start_node;
    
    for k in graph.graph.get(&grid[0][0]).unwrap().iter() {
        println!("b {} {}", k, check_bottom(&mut grid[0][0], &mut k.clone()));
        println!("r {} {}", k, check_right(&mut grid[0][0], &mut k.clone()));
        // println!("r {} {}", k, check_right(&mut grid[0][0], &mut k.clone()));

    }
    for i in 0..grid.len()-1 {
        for j in 0..grid[0].len()-1 {
            println!("grid[{}][{}]: {}",i,j, grid[i][j]);
            for k in graph.graph.get(&grid[i][j]).unwrap().iter() {
                // if i == 0 && j == 0 {
                //     continue;
                // }
                // if i == 0 {
                    
                    if !visited.contains(&k) {
                        println!("  k: {}", k);
                        let mut temp = k.clone();
                        if check_right(&mut grid[i][j].clone(), &mut temp) {
                            grid[i][j+1] = temp.clone();
                            println!("grid1 {}", grid[i][j+1]);
                            visited.insert(grid[i][j+1].clone());
                            // break;
                        }
                        let mut temp = k.clone();
                        if check_bottom(&mut grid[i][j].clone(), &mut temp) {
                            grid[i+1][j] = temp.clone();
                            println!("grid2 {}", grid[i+1][j]);
                            visited.insert(grid[i+1][j].clone());
                            // break;
                        }
                        // println!("temp {}", temp);
                    }
                // }
                // else {
                //     if !visited.contains(&k) {
                //         grid[i+1][j] = k.clone();
                //         if check_bottom(&mut grid[i][j].clone(), &mut grid[i+1][j]) {
                //             visited.insert(grid[i+1][j].clone());
                //             break;
                //         }
                //     }
                    
                // }
            }
        }
    }
    
    // grid[1][0] = start_node.clone();
    // build_grid(&start_node, &mut graph, &mut visited, &mut grid, 1, 0); //vec![vec!['?'; 3]; 3];
    for i in grid.iter() {
        for j in i.iter() {
            print!("{} ", j.id);
        }
        println!("");
    }
    
    Ok(2) // 363
}

fn build_grid(
    node: &Node,
    graph: &mut Graph,
    visited: &mut HashSet<Node>,
    grid: &mut Vec<Vec<Node>>,
    y: usize,
    x: usize,
)  {
    // println!("{} {}",y, x);
    visited.insert(node.clone());
    // if x >= 3 || y >= 3 {
    //     return;
    // }
    // println!("G");
    let mut flag = true;
    for i in 0..graph.graph.get(&node).unwrap().len() {
        flag = true;
        if !visited.contains(&graph.graph.get(&node).unwrap()[i]) {
            
            if y < 3 {
                println!("grid[{}][{}] = {}", y, x, graph.graph.get(&node).unwrap()[i].clone());
                grid[y][x] = graph.graph.get(&node).unwrap()[i].clone();
                visited.insert(graph.graph.get(&node).unwrap()[i].clone());
                flag = false;
                build_grid(&graph.graph.get(&node).unwrap()[i].clone(), graph, visited, grid, y + 1, x);
            }
            else {
                build_grid(&graph.graph.get(&node).unwrap()[i].clone(), graph, visited, grid, 0, x + 1);
            }
        }
        // println!("hm");
        
    }
    // if flag {
    //     build_grid(node, graph, visited, grid, 0, x + 1);
    // }
    // graph.graph(node);
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file_batch(String::from("input/test/day20.txt"));
        assert_eq!(crate::day20::part_one(&vec), Ok(20899048083289));
    }

    // #[test]
    // fn part_one_actual() {
    //     let vec = crate::readfile::fileio::read_file_batch(String::from("input/day20.txt"));
    //     assert_eq!(crate::day20::part_one(&vec), Ok(276));
    // }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file_batch(String::from("input/test/day20.txt"));
        assert_eq!(crate::day20::part_two(&vec), Ok(848));
    }

    // #[test]
    // fn part_two_actual() {
    //     let vec = crate::readfile::fileio::read_file_batch(String::from("input/day20.txt"));
    //     assert_eq!(crate::day20::part_two(&vec), Ok(2136));
    // }
}
