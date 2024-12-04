use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let mut file = File::open("./input.txt").unwrap();
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

    let list: Vec<Vec<i32>> = buf
        .lines()
        .map(|x| x.split_whitespace().map(|x| x.parse().unwrap()).collect())
        .collect();

    let (mut list1, mut list2): (Vec<i32>, Vec<i32>) = list.iter().map(|x| (x[0], x[1])).unzip();

    println!("PART ONE:");

    list1.sort();
    list2.sort();

    let distances: i32 = list1
        .iter()
        .zip(list2.iter())
        .map(|x| (x.0 - x.1).abs())
        .sum();

    println!("Distances = {}", &distances);

    println!("PART TWO:");

    let counts: HashMap<i32, i32> = list2.iter().fold(HashMap::new(), |mut acc, &e| {
        *acc.entry(e).or_insert(0) += 1;
        acc
    });

    let similarity_score: i32 = list1.iter().map(|x| x * counts.get(x).unwrap_or(&0)).sum();
    println!("Similarity score = {}", similarity_score);
}
