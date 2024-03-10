use std::collections::HashMap;
fn main() {
    let mut h: HashMap<char, i32> = HashMap::new();
    for (i, c) in "hello!".chars().enumerate() {
        h.entry(c).or_insert(i.try_into().unwrap());
    }
    println!("{:#?}", h);
    let mut sum = 0;
    for i in h {
        if i.0 == 'l' {
            sum += i.1;
        }
    }
    println!("{}", sum);
}
