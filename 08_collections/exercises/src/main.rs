use std::collections::HashMap;

fn main() {
    exercise1();
}

// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode
// (the value that occurs most often; a hash map will be helpful here) of the list.

fn exercise1() {
    let mut nums = vec![1, 2, 9, 3, 4, 8, 5, 1, 2, 0, 2];
    nums.sort();
    let median = nums[nums.len() / 2];

    let mut map = HashMap::new();

    for num in &nums {
        *map.entry(num).or_insert(0) += 1
    }

    let mut highest = 0;
    let mut mode = 0;
    for (key, value) in &map {
        if value > &highest {
            highest = *value;
            mode = **key;
        }
    }

    println!("{:?}", nums);
    println!("{:?}", median);
    println!("{:?}", map);
    println!("{:?}", mode)
}




























// fn exercise1() {
//     let values: Vec<i32> = vec![1, 2, 5, 6, 3, 4, 5, 4, 4];
//     println!("{:?}", values);
//     println!("{}", find_median(&values));
//     println!("{}", find_mode(&values));
// }

// fn find_median(list: &Vec<i32>) -> i32 {
//     match list.len() {
//         0 => 0,
//         other => {
//             let mut sorted = list.clone();
//             sorted.sort();
//             sorted[other / 2]
//         }
//     }
// }

// fn find_mode(list: &Vec<i32>) -> i32 {
//     let mut map = HashMap::new();
//     for val in list {
//         let previous = map.entry(val).or_insert(0);
//         *previous += 1;
//     }
//     **map
//         .iter()
//         .max_by_key(|entry| entry.1)
//         .map(|(k, _v)| k)
//         .unwrap()
// }

// fn exercise2() {
//     let input = "lorem ipsum dolet amor";
//     println!("{}", piglatin(input));
// }

// fn piglatin(input: &str) -> String {
//     let vowels = ['a', 'e', 'i', 'o', 'u'];
//     let mut result = Vec::new();
//     for word in input.split_whitespace() {
//         let first = word.chars().next().unwrap();
//         if vowels.contains(&first) {
//             result.push(format!("{}hay", word));
//         } else {
//             result.push(format!("{}{}ay", &word[1..], first));
//         }
//     }
//     result.join(" ")
// }

// fn exercise3() {
//     let commands = [
//         "Add Sally to Engineering",
//         "Add Amir to Sales",
//         "Add John to Sales",
//         "List Engineering",
//         "List Sales",
//         "List All",
//     ];

//     let mut context = HashMap::new();

//     for command in commands {
//         run_command(command.to_string(), &mut context)
//     }
// }

// fn run_command(command: String, context: &mut HashMap<String, Vec<String>>) -> () {
//     let tokens: Vec<&str> = command.split(" ").collect();

//     match &tokens[..] {
//         ["Add", name, "to", department] => {
//             let existing = context.entry(department.to_string()).or_insert(Vec::new());
//             existing.push(name.to_string());
//         }
//         ["List", "All"] => println!("{:?}", context),
//         ["List", department] => match context.get(&department[..]) {
//             Some(res) => println!("{:?}", res),
//             None => println!("Department not found"),
//         },
//         _ => panic!("Could not parse command"),
//     }
// }
