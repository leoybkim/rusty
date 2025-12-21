use std::collections::HashMap;
use std::io;

fn main() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    let num = v1.pop();
    println!("Popped: {:?}", num);

    let v2 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v3 = vec![100, 32, 57];
    for i in &v3 {
        println!("{i}");
    }

    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
    }

    println!("v4: {:?}", v4);

    

    let mut s1 = String::new();
    s1.push_str("foo");
    let data = "initial contents";
    let s2 = data.to_string();
    let s2 = "initial content".to_string();
    let s2 = String::from("initial contents");

    let s3 = "bar";
    s1.push_str(s3);
    println!("s3 is {s3}");

    let s4 = s1 + &s2;
    println!("s4 is {s4}");
    let s = format!("{s2}-{s3}-{s4}");
    println!("s is {s}");
    

    let hello = String::from("Здравствуйте");
    //let answer = &hello[0..2];
    let len_hello = hello.len();
    println!("{len_hello}");
    
    
    let hello2 = String::from("नमस्ते");
    let answer2 = &hello2[0..3];
    let len_hello2 = hello2.len();
    println!("{len_hello2}");
    println!("{answer2}");
    
    
    for b in "नमस्ते". bytes() {
       println!("{b}");
    }

    
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    let blue_team_name = String::from("blue");
    let blue_team_score = scores.get(&blue_team_name).copied().unwrap_or(0);

    println!("blue team score: {blue_team_score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let green = String::from("green");
    scores.insert(green, 100);

    scores.insert(String::from("blue"), 20);

    println!("{scores:?}");

    scores.entry(String::from("blue")).or_insert(50);
    scores.entry(String::from("purple")).or_insert(50);
    println!("{scores:?}");


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");


    // Exercises
    // 1. Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    let mut int_list = vec![0, 1, 2, 3, 4, 5, 6, 7, 10, 50, 100, 10000, 42, 11, 11, 11];
    int_list.sort();
    println!("list of integers: {int_list:?}");

    let median = find_median(&int_list);
    println!("Median: {:.2}", median);

    let (mode, count) = find_mode(&int_list);
    println!("Mode: {} (occurs {} times)", mode, count);


    // 2. Convert strings to Pig Latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
    let first = String::from("first");
    let apple = String::from("apple");

    let pig_latin_first = pig_latin(&first);
    let pig_latin_apple = pig_latin(&apple);

    println!("pig latin {first}: {pig_latin_first}");
    println!("pig latin {apple}: {pig_latin_apple}");
    

    // 3. Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then, let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

    let mut directory: HashMap<String, Vec<String>> = HashMap::new();

    println!("Starting Directory Program");
    println!("Commands: ");
    println!("  Add [Name] to [Department]");
    println!("  List [Department]");
    println!("  List all");
    println!("  Quit");

    loop {
        println!("\n\nEnter command: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            println!("Exiting");
            break;
        }

        if input.starts_with("Add ") {
            if let Some((name, department)) = parse_add_command(input) {
                let name = name.to_string();
                let department = department.to_string();

                directory.entry(department.clone()).or_insert(Vec::new()).push(name.clone());
                println!("Added {name} to {department}");
            }
        } else if input.starts_with("List all") {
            list_all(&directory);
        } else if input.starts_with("List ") {
            if let Some(department) = parse_list_command(input) {
                list_department(&directory, department);
            } else {
                println!("Invalid command. Did you mean List [Department]?");
            }
        } else {
            println!("Invalid command.");
        }
    }
}


fn parse_add_command(command: &str) -> Option<(&str, &str)> {
    if let Some((before_to, after_to)) = command[4..].split_once(" to ") {
        Some((before_to.trim(), after_to.trim()))
    } else {
        None
    }
}

fn parse_list_command(command: &str) -> Option<&str> {
    Some(command.strip_prefix("List ")?.trim())
}

fn list_department(directory: &HashMap<String, Vec<String>>, department: &str) {
    if let Some(people) = directory.get(department) {
        let mut people = people.clone();
        people.sort();
        println!("People in {department}: ");
        for person in people {
            println!("{person}");
        }
    } else {
        println!("Department: {department} not found");
    }
}

fn list_all(directory: &HashMap<String, Vec<String>>) {
    let mut departments: Vec<_> = directory.keys().collect();
    departments.sort();

    println!("Listing by departments: ");
    for department in departments {
        list_department(directory, department);
    }
}


fn find_median(int_list: &[i32]) -> f64 {
    let len = int_list.len();
    if len % 2 == 0 {
        (int_list[len/2 - 1] + int_list[len/2]) as f64 / 2.0
    } else {
        int_list[len/2] as f64
    }
}

fn find_mode(int_list: &[i32]) -> (i32, usize) {
    let mut count_map = HashMap::new();
    for &num in int_list {
        *count_map.entry(num).or_insert(0) += 1;
    }

    count_map.into_iter()
        .max_by_key(|(_, count)| *count)
        .unwrap()
}

fn pig_latin(word: &str) -> String {
    match word.chars().next() {
        Some(first_char) => {
            let rest = &word[first_char.len_utf8()..];
            format!("{}{}ay", rest, first_char)
        }
        None => String::new(),
    }
}
