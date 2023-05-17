use std::fs;

fn main() {
    day1();
}

fn day1() {
    let file = fs::read_to_string("./data/day1.data").expect("Missing: day1.data");

    let bags = file.split("\n\n");

    let mut totals: Vec<u16> = bags
        .into_iter()
        .map(|bag| {
            bag.lines()
                .map(|val| val.parse::<u16>().unwrap())
                .reduce(|acc, v| acc + v)
                .unwrap()
        })
        .collect();
    totals.sort();

    let res_1 = &totals.iter().max().unwrap();
    println!("part 1: {:?}", res_1);

    let res_2 = &totals.iter().rev().take(3);
    println!("part 2: {:?}", res_2);
}
