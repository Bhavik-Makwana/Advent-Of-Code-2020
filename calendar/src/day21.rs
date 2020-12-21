use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Error;

pub fn part_one(input: &Vec<String>) -> Result<i32, Error> {
    let mut meals = Vec::new();
    for line in input.iter() {
        let x = line
            .split(" (contains ")
            .map(|x| String::from(x))
            .collect::<Vec<String>>();
        let i: HashSet<String> = x[0]
            .split(" ")
            .map(|x| String::from(x))
            .collect::<HashSet<String>>();
        let a: HashSet<String> = x[1]
            .replace(",", "")
            .replace(")", "")
            .split(" ")
            .map(|x| String::from(x))
            .collect::<HashSet<String>>();
        // println!("{:?} {:?}", i, a);
        meals.push(vec![i, a]);
    }

    let mut all_ingredients = HashSet::new();
    let mut all_allergens = HashSet::new();

    for meal in meals.iter() {
        let uni: HashSet<&String> = all_allergens.union(&meal[1]).collect::<HashSet<&String>>();
        all_allergens = uni.into_iter().map(|x| x.clone()).collect();

        let uni: HashSet<&String> = all_ingredients
            .union(&meal[0])
            .collect::<HashSet<&String>>();
        all_ingredients = uni.into_iter().map(|x| x.clone()).collect();
    }
    // println!("{:?}", all_allergens);
    // println!("{:?}", all_ingredients);
    let mut bad: HashMap<String, HashSet<String>> = HashMap::new();
    let mut count: HashMap<String, i32> = HashMap::new();
    for i in all_ingredients.iter() {
        bad.insert(i.to_string(), all_allergens.clone());
        count.insert(i.to_string(), 0);
    }

    for meal in meals.iter() {
        for ingredient in meal[0].iter() {
            count.insert(ingredient.to_string(), count.get(ingredient).unwrap() + 1);
        }

        for allergy in meal[1].iter() {
            for ingredient in all_ingredients.iter() {
                if !meal[0].contains(ingredient) {
                    bad.insert(
                        ingredient.to_string(),
                        bad.get(ingredient)
                            .unwrap()
                            .into_iter()
                            .filter(|x| *x != allergy)
                            .map(|x| x.to_string())
                            .collect::<HashSet<String>>(),
                    );
                }
            }
        }
    }

    let mut ans = 0;
    for ingredient in all_ingredients.iter() {
        if bad.get(ingredient).unwrap().len() == 0 {
            // println!("{}", count.get(ingredient).unwrap());
            ans += count.get(ingredient).unwrap();
        }
    }
    
    Ok(ans) // 156
}

pub fn part_two(input: &Vec<String>) -> Result<String, Error> {
    let mut meals = Vec::new();
    for line in input.iter() {
        let x = line
            .split(" (contains ")
            .map(|x| String::from(x))
            .collect::<Vec<String>>();
        let i: HashSet<String> = x[0]
            .split(" ")
            .map(|x| String::from(x))
            .collect::<HashSet<String>>();
        let a: HashSet<String> = x[1]
            .replace(",", "")
            .replace(")", "")
            .split(" ")
            .map(|x| String::from(x))
            .collect::<HashSet<String>>();
        meals.push(vec![i, a]);
    }

    let mut all_ingredients = HashSet::new();
    let mut all_allergens = HashSet::new();

    for meal in meals.iter() {
        let uni: HashSet<&String> = all_allergens.union(&meal[1]).collect::<HashSet<&String>>();
        all_allergens = uni.into_iter().map(|x| x.clone()).collect();

        let uni: HashSet<&String> = all_ingredients
            .union(&meal[0])
            .collect::<HashSet<&String>>();
        all_ingredients = uni.into_iter().map(|x| x.clone()).collect();
    }
    // println!("{:?}", all_allergens);
    // println!("{:?}", all_ingredients);
    let mut bad: HashMap<String, HashSet<String>> = HashMap::new();
    let mut count: HashMap<String, i32> = HashMap::new();
    for i in all_ingredients.iter() {
        bad.insert(i.to_string(), all_allergens.clone());
        count.insert(i.to_string(), 0);
    }

    for meal in meals.iter() {
        for ingredient in meal[0].iter() {
            count.insert(ingredient.to_string(), count.get(ingredient).unwrap() + 1);
        }

        for allergy in meal[1].iter() {
            for ingredient in all_ingredients.iter() {
                if !meal[0].contains(ingredient) {
                    bad.insert(
                        ingredient.to_string(),
                        bad.get(ingredient)
                            .unwrap()
                            .into_iter()
                            .filter(|x| *x != allergy)
                            .map(|x| x.to_string())
                            .collect::<HashSet<String>>(),
                    );
                }
            }
        }
    }

    let mut mapping = HashMap::new();
    let mut ans = 0;
    for ingredient in all_ingredients.iter() {
        if bad.get(ingredient).unwrap().len() == 0 {
            // println!("{}", count.get(ingredient).unwrap());
            ans += count.get(ingredient).unwrap();
        } else if bad.get(ingredient).unwrap().len() == 1 {
            mapping.insert(
                ingredient.to_string(),
                bad.get(ingredient).unwrap().iter().next().unwrap(),
            );
        }
    }

    let mut identified: HashSet<String> = HashSet::new();

    while mapping.len() < all_allergens.len() {
        for ingredient in all_ingredients.iter() {
            let sus = bad
                .get(ingredient)
                .unwrap()
                .iter()
                .filter(|a| !identified.contains(a.clone()))
                .collect::<Vec<&String>>();
            // [allergen for allergen in bad[ingredient] if allergen not in identified]
            if sus.len() == 1 {
                mapping.insert(ingredient.to_string(), sus[0]);
                identified.insert(sus[0].to_string());
                // identified.add(sus[0]);
            }
        }
    }
    // println!("{:?}", mapping);
    let mut x: Vec<_> = mapping.iter().collect();
    x.sort_by(|a, b| b.1.cmp(a.1));
    x.reverse();

    let s = x
        .iter()
        .map(|(x, y)| x)
        .map(|x| x.to_string() + ",")
        .collect::<String>();

    Ok(s[0..s.len() - 1].to_string()) // 363
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day21.txt"));
        assert_eq!(crate::day21::part_one(&vec), Ok(5));
    }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day21.txt"));
        assert_eq!(crate::day21::part_one(&vec), Ok(2280));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day21.txt"));
        assert_eq!(
            crate::day21::part_two(&vec),
            Ok(String::from("mxmxvkd,sqjhc,fvjkl"))
        );
    }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day21.txt"));
        assert_eq!(
            crate::day21::part_two(&vec),
            Ok(String::from(
                "vfvvnm,bvgm,rdksxt,xknb,hxntcz,bktzrz,srzqtccv,gbtmdb"
            ))
        );
    }
}
