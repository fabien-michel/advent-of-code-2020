use crate::utils::print_day_banner;
use crate::utils::read_lines;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
// use itertools::iproduct;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Ingredient {
    name: String,
    allergens_candidates: Vec<String>,
}

#[derive(Debug, Clone)]
struct Food {
    // ingredients: Vec<Ingredient>,
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

pub fn day21_01() {
    print_day_banner(21, 1);

    let foods = load_foods();

    let allergens_ingredients: Vec<_> =
        resolve_allergens_ingredient(&get_allergens_possible_ingredients(&foods))
            .values()
            .cloned()
            .collect();
    let ingredient_without_allergen_count: usize = foods
        .iter()
        .map(|food| {
            food.ingredients
                .iter()
                .filter(|i| !allergens_ingredients.contains(i))
                .count()
        })
        .sum();

    println!("{:?}", ingredient_without_allergen_count);
}
pub fn day21_02() {
    print_day_banner(21, 2);
    let foods = load_foods();

    let allergens_ingredients =
        resolve_allergens_ingredient(&get_allergens_possible_ingredients(&foods));

    let mut allergens: Vec<_> = allergens_ingredients.keys().collect();
    allergens.sort();
    let ingredients_list = allergens
        .iter()
        .map(|a| allergens_ingredients.get(*a).unwrap())
        .join(",");

    println!("{:?}", ingredients_list);
}

fn get_allergens_possible_ingredients(foods: &Vec<Food>) -> HashMap<String, HashSet<String>> {
    let mut allergens_ingredients: HashMap<String, HashSet<String>> = HashMap::new();

    for food in foods.iter() {
        let ingredients_set: HashSet<String> = food.ingredients.iter().cloned().collect();
        for allergen in food.allergens.iter() {
            let allergen_ingredients = allergens_ingredients.get(allergen);
            if allergen_ingredients.is_none() {
                allergens_ingredients.insert(allergen.clone(), ingredients_set.clone());
            } else {
                let intersecting_ingredients = allergen_ingredients
                    .unwrap()
                    .intersection(&ingredients_set)
                    .cloned()
                    .collect();
                allergens_ingredients.insert(allergen.clone(), intersecting_ingredients);
            }
        }
    }
    return allergens_ingredients;
}

fn resolve_allergens_ingredient(
    allergens_ingredients: &HashMap<String, HashSet<String>>,
) -> HashMap<String, String> {
    // let mut ok_ai: HashMap<String, String> = HashMap::new();
    let mut result: HashMap<String, String> = HashMap::new();
    let mut known_ingredients: HashSet<String> = HashSet::new();
    let mut still = true;
    let mut new_allergens_ingredients = allergens_ingredients.clone();
    while still {
        still = false;
        let allergens_ingredients = new_allergens_ingredients.clone();
        for (allergen, ingredients) in allergens_ingredients {
            if ingredients.len() == 1 {
                result.insert(allergen.clone(), ingredients.iter().nth(0).unwrap().clone());
                known_ingredients.insert(ingredients.iter().nth(0).unwrap().clone());
                continue;
            }
            still = true;
            let unknown_ingredients: HashSet<String> = ingredients
                .difference(&known_ingredients)
                .cloned()
                .collect();
            new_allergens_ingredients.insert(allergen.clone(), unknown_ingredients);
        }
    }

    return result;
}

fn load_foods() -> Vec<Food> {
    return read_lines("./inputs/21")
        .unwrap()
        .filter_map(Result::ok)
        .map(parse_line)
        .collect();
}

fn parse_line(line: String) -> Food {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(?P<ingredients>.+) \(contains (?P<allergens>.+)\)").unwrap();
    }

    let caps = RE.captures(line.as_str()).unwrap();
    let allergens: Vec<String> = caps
        .name("allergens")
        .unwrap()
        .as_str()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    let ingredients: Vec<String> = caps
        .name("ingredients")
        .unwrap()
        .as_str()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    return Food {
        ingredients: ingredients,
        allergens: allergens,
    };
}
