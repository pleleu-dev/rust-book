// use std::collections::HashMap;

fn mystery<T>(x: T) -> T {
    x + x
}

fn main() {
    /*
       Given a list of integers, use a vector and return the median
       (when sorted, the value in the middle position)
       and mode (the value that occurs most often;
       a hash map will be helpful here) of the list.
    */
    let list = vec![12, 65, 99, 12, 45, 1, 56, 43, 99, 12, 88, 99, 99, 99];
    println!("median {}", find_median(&list));
    println!("list {:?}", mystery(3))
}

// fn find_mode(list: &[i32; 14]) -> i32 {
//     let mut hash_map = HashMap::new();

//     let mut max_entry = list[0];
//     let mut max_value = 0;

//     for i in list {
//         hash_map
//             .entry(i)
//             .and_modify(|counter| *counter += 1)
//             .or_insert(1);
//     }

//     for c in hash_map {
//         let val: i32 = c.1;

//         if max_value < val {
//             max_entry = c.0;
//             max_value = c.1
//         }
//     }
//     max_entry
// }

fn find_median(list: &Vec<i32>) -> i32 {
    let mut v = list.clone();
    v.sort();
    let med = v.len() / 2;

    *&v[med]
}

// fn capitalize_first_letter(s: String) -> String {
//     let first_letter = s.split_at(1);
//     format!("{}{}", first_letter.0.to_uppercase(), first_letter.1)
// }

// fn main() {
//     /*
//     Convert strings to pig latin.
//     The first consonant of each word is moved to the end of the word and “ay” is added,
//     so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead
//     (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
//      */
//     let s = String::from("first apple");
//     let mut result = String::new();

//     for word in s.split_whitespace() {
//         let first_letter = word.split_at(1);
//         if "aeiouy".contains(first_letter.0) {
//             result.push_str(&format!("{}{}-hay ", first_letter.0, first_letter.1))
//         } else {
//             result.push_str(&format!(
//                 "{}-{}ay ",
//                 first_letter.1,
//                 first_letter.0.to_lowercase()
//             ))
//         }
//     }

//     println!("result {}", capitalize_first_letter(result))
// }

// use std::collections::HashMap;
// use std::io;

// fn add_to_department(
//     name: Option<&str>,
//     place: Option<&str>,
//     company: &mut HashMap<String, Vec<String>>,
// ) {
//     let name_or_empty = name.unwrap_or("");
//     let place_or_empty = place.unwrap_or("");

//     if name_or_empty.is_empty() || place_or_empty.is_empty() {
//         println!("Wrong input")
//     } else {
//         let names = company
//             .entry(place_or_empty.to_string())
//             .or_insert(Vec::new());
//         names.push(name_or_empty.to_string());

//         println!("Adding name: {:?} to {:?}", name.unwrap(), place.unwrap());
//     }
// }

// fn list_all_in(place: Option<&str>, company: &HashMap<String, Vec<String>>) {
//     let place_or_empty = place.unwrap_or("");

//     if place_or_empty.is_empty() {
//         println!("Wrong input")
//     } else {
//         if place_or_empty == "company" {
//             println!("Company employees : {:?}", company);
//         } else {
//             println!(
//                 "{} employees: {:?}",
//                 place_or_empty,
//                 company.get(place_or_empty).unwrap_or(&Vec::new())
//             );
//         }
//     }
// }

// fn main() {
//     /*
//     Using a hash map and vectors,
//     create a text interface to allow a user to add employee names to a department in a company.
//     For example, “Add Sally to Engineering” or “Add Amir to Sales.”
//     Then let the user retrieve a list of all people in a department or all people in the company by department,
//     sorted alphabetically
//      */
//     let mut company: HashMap<String, Vec<String>> = HashMap::new();

//     println!("Welcome");

//     loop {
//         println!("What do you want ?");

//         let mut input: String = String::new();
//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");

//         let mut input_iter = input.split_whitespace();
//         let action = input_iter.next();
//         let name = input_iter.next();
//         input_iter.next();
//         let place = input_iter.next();

//         match action {
//             Some(a) => match a.to_lowercase().as_str() {
//                 "add" => add_to_department(name, place, &mut company),
//                 "list" => list_all_in(place, &company),
//                 "quit" => break,
//                 _ => println!("Wrong input"),
//             },
//             None => {
//                 println!("Wrong input")
//             }
//         }
//     }
// }
