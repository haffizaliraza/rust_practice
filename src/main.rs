use std::io;
use rand::Rng;


pub struct Tweet {
    pub text: String,
    pub username: String,
    pub retweet: bool,
    pub reply: bool
}

impl Summery for Tweet {
    fn summerize(&self) -> String {
        return format!("{}, by {}",self.text, self.username)
    }
}

pub struct Article {
    pub title: String,
    pub content: String,
    pub author: String
}

impl Summery for Article {
    fn summerize(&self) -> String {
        return format!("{}, by {}",self.title, self.author)
    }
}

pub trait Summery {
    fn summerize(&self) -> String{
        String::from("Read More...")
    }
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest_numnber = list[0];

    for i in list {
        if *i > largest_numnber {
            largest_numnber = *i;
        }
    }
    return largest_numnber;
}


fn get_largest_char(list: &[char]) -> char {
    let mut largest_char = list[0];

    for i in list {
        if *i > largest_char {
            largest_char = *i;
        }
    }
    return largest_char;
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}


fn main() {
    let my_string = String::from("Hello world");
    let word = first_word(&my_string);
    println!("first word is: {}", word);
    let string1 = String::from("hello world");
    let string2 = String::from("This is a test");
    let result = longest(&string1, &string2);
    println!("longest string is {}", result);


    let char_list= vec!['a', 'n', 'l', 'w', 'e', 'f'];
    println!("largest char: {}", largest(&char_list));
    let num_list = vec![1,6,7,4,33,99,7,678,34,54,77];
    println!("largest num: {}", largest(&num_list));

    let tweet = Tweet {
        text: String::from("Hello, World!"),
        username: String::from("user123"),
        retweet: true,
        reply: false
    };

    let article = Article {
        title: String::from("My First Rust Program"),
        content: String::from("This is my first Rust program"),
        author: String::from("John Doe")
    };

    println!("Summarized Tweet: {}", tweet.summerize());
    println!("Summerized article: {}", article.summerize());
    

    let fruits: [&str; 5] = ["apple", "orange", "banana", "pineapple", "guava"];
    let selected_fruit = fruits[rand::thread_rng().gen_range(0..fruits.len())];

    
    let mut guess = String::new();


    while guess.trim().to_lowercase() != selected_fruit {
        guess.clear();
        println!("Enter Fruit Name");
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        if guess.trim().to_lowercase() == "exit" {
            println!("Exiting the game");
            break;
        }
    }

    println!("You gessed {}", guess.to_lowercase());
}