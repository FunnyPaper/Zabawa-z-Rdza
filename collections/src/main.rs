fn main() {
    let v1 = Vec::<u32>::new();
    let v2: Vec<u32> = Vec::new();
    let mut v3 = Vec::new();
    v3.push(0u32);

    // immutable referencja -> nie można przypisać nic innego do x
    let x: &mut Vec<u32> = &mut v3; // x ma dostęp do v3 bez zmiany ownera
    x.push(7); 

    // mutable referencja -> można przypisać coś innego do x
    let mut x: &Vec<u32> = &v1; // x ma dostęp do v1 bez zmiany ownera
    x = &v2; // x ma dostęp di v2 bez zmiany ownera

    // mutable zmienna
    let mut x: Vec<u32> = v3; // v3 staje się invalid
    // v3.push(7);
    
    x = v1; // v1 staje się invalid
    x = v2; // v2 staje się invalid

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{s1}{s2}");
    // let s3 = String::new() + &s1 + &s2;
    // let s3 = s1 + &s2;

    println!("content of:\n[s1] -> [{s1}]\n[s2] -> [{s2}]\n[s3] -> [{s3}]");

    // ---------------------------------------------------------------------------

    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
    teams.into_iter().zip(initial_scores.into_iter()).collect();

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let mut num = 32u8;
    let ref_num = &mut num;
    *ref_num += 1;

    println!("Value of num is {}", &num);
}
