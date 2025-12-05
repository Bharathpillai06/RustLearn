    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

fn main() {
    // stores empty cause type is initialized
    let v: Vec<i32> = Vec::new();

    // using vec! it auto fills type :)
    let v = vec![1, 2, 3];

    //modifing vector using 

    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

        let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; //returns an error if index is not in array
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2); //returnns non if index isnt in array
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),

    }
    //imutable traversel of the vector refrences
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    //mutable traveserl through the array refrences this add 50 to each element
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }



    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    let mut s = String::new();
        let s = String::from("initial contents");
    
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // format uses references so that this call doesn’t take ownership of any of its parameters.
    let s = format!("{s1}-{s2}-{s3}");

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    //insert is a static method that takes in two values of the key and the value pair
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // to get value use get method with parameter of the the key
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    //Here, score will have the value that’s associated with the Blue team, 
    //and the result will be 10. The get method returns an Option<&V>; 
    //if there’s no value for that key in the hash map, get will return None. 
    //This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, 
    //then unwrap_or to set score to zero if scores doesn’t have an entry for the key.

    //traversal of a hash map is the same as a vector

    for (key, value) in &scores {
        println!("{key}: {value}");
    }


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

    


}


