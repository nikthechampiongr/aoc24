use std::io::Read;

fn main() {
    let mut total = 0;

    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();
    let mut input = std::fs::File::open("input.txt").unwrap();
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();

    for line in buf.lines() {
        let mut split = line.split_whitespace();
        list1.push(split.next().unwrap().parse().unwrap());
        list2.push(split.next().unwrap().parse().unwrap());
        assert_eq!(split.next(), None);
    }
    list1.sort();
    list2.sort();

    for i in 0..list1.len() {
        total += list1[i].abs_diff(list2[i]);
    }
    println!("Total distance: {total}");

    let mut similar = 0;

    let mut prev = 0;
    let mut prev_inc = 0;

    for id in list1 {
        if id == prev {
            similar += prev_inc;
            continue;
        }

        prev = id;

        prev_inc = list2.iter().filter(|id| **id == prev).count() * prev as usize;
        similar += prev_inc;
    }

    println!("Similarity score: {similar}");
}
