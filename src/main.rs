
use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use csv::Writer;

#[derive(Debug)]
struct Site {
    url: &'static str,
    products_div: &'static str,
    image: &'static str,
    product_title: &'static str,
    old_price: &'static str,
    new_price: &'static str,
    product_url: &'static str,
    brand_name: &'static str,
    image_attribute: &'static str,
    category: &'static str,
}

fn main() -> Result<(), Box<dyn Error>> {
    let sites = vec![
        Site {
            url: "https://www.limelight.pk/collections/winter-sale",
            products_div: "ul#main-collection-product-grid",
            image: "img",
            product_title: "a.full-unstyled-link span",
            old_price: "span.price-item--regular span",
            new_price: "span.price-item--sale span",
            product_url: "a.full-unstyled-link",
            brand_name: "limelight",
            image_attribute: "src",
            category: "Winter Women",
        },
        Site {
            url: "https://www.limelight.pk/collections/western-sale",
            products_div: "ul#main-collection-product-grid",
            image: "img",
            product_title: "a.full-unstyled-link span",
            old_price: "span.price-item--regular span",
            new_price: "span.price-item--sale span",
            product_url: "a.full-unstyled-link",
            brand_name: "limelight",
            image_attribute: "src",
            category: "Western Women",
        },
        // Add other site entries here...
    ];

    let mut wtr = Writer::from_writer(File::create("products.csv")?);

    for site in sites {
        let response = get(site.url)?;
        let body = response.text()?;
        let document = Html::parse_document(&body);
        let products_selector = Selector::parse(site.products_div).unwrap();

        for product in document.select(&products_selector) {
            let name = extract_text(&product, site.product_title);
            let url = extract_attr(&product, site.product_url, "href");
            let image = extract_attr(&product, site.image, site.image_attribute);
            let old_price = extract_text(&product, site.old_price);
            let new_price = extract_text(&product, site.new_price);

            if let (Some(name), Some(url), Some(image), Some(old_price), Some(new_price)) = (name, url, image, old_price, new_price) {
                wtr.write_record(&[
                    name.clone(),
                    url,
                    image,
                    old_price,
                    new_price,
                    site.brand_name.to_string(),
                    site.category.to_string(),
                ])?;
                println!("Product Created: {}", name);
            }
        }
    }

    wtr.flush()?;
    Ok(())
}

fn extract_text(product: &scraper::ElementRef, selector_str: &str) -> Option<String> {
    let selector = Selector::parse(selector_str).ok()?;
    product.select(&selector).next()?.text().collect::<Vec<_>>().join(" ").trim().to_string().into()
}

fn extract_attr(product: &scraper::ElementRef, selector_str: &str, attribute: &str) -> Option<String> {
    let selector = Selector::parse(selector_str).ok()?;
    product.select(&selector).next()?.value().attr(attribute).map(|attr| attr.to_string())
}




// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;


// pub struct Tweet {
//     pub text: String,
//     pub username: String,
//     pub retweet: bool,
//     pub reply: bool
// }

// impl Summery for Tweet {
//     fn summerize(&self) -> String {
//         return format!("{}, by {}",self.text, self.username)
//     }
// }

// pub struct Article {
//     pub title: String,
//     pub content: String,
//     pub author: String
// }

// impl Summery for Article {
//     fn summerize(&self) -> String {
//         return format!("{}, by {}",self.title, self.author)
//     }
// }

// pub trait Summery {
//     fn summerize(&self) -> String{
//         String::from("Read More...")
//     }
// }

// fn largest_i32(list: &[i32]) -> i32 {
//     let mut largest_numnber = list[0];

//     for i in list {
//         if *i > largest_numnber {
//             largest_numnber = *i;
//         }
//     }
//     return largest_numnber;
// }


// fn get_largest_char(list: &[char]) -> char {
//     let mut largest_char = list[0];

//     for i in list {
//         if *i > largest_char {
//             largest_char = *i;
//         }
//     }
//     return largest_char;
// }

// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }


// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     return &s[..];
// }

// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }


// fn main() {
//     let secret_number = rand::thread_rng().gen_range(1..=100);
//     let nn = b'A';
//     let heart_eyed_cat = '😻';
//     let tt: (i32, f32) = (0, 0.5);
//     println!("u8 {}, {}, {:?}", nn, heart_eyed_cat, tt);
//     loop {
//         println!("Please Enter your guess!");
//         let mut guess_number = String::new();

//         io::stdin()
//         .read_line(&mut guess_number)
//         .expect("Failed to read line");
    
//         let guess_number: u32 = match guess_number.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         match guess_number.cmp(&secret_number) {
//             Ordering::Less => println!("Too Less"),
//             Ordering::Greater => println!("Too Greater"),
//             Ordering::Equal => {
//                 println!("{}", guess_number);
//                 break;
//             }
//         }

//     }

//     let localhost = IpAddr{
//         kind: IpAddrKind::V4(127, 0, 0, 1),
//         address: String::from("127.0.0.1"),
//     };

//     let localhost = IpAddrKind::V4(127, 0, 0, 1);

//     let x = 5;
//     let y = 10;

//     assert!(x < y, "x should be less than y");
//     // assert!(x > y, "x should not be greater than y");
//     let a = 3;
//     let b = 1 + 2;
//     assert_eq!(a, b);

//     assert_eq!(a, b, "we are testing addition with {} and {}", a, b);

//     let my_string = String::from("Hello world");
//     let word = first_word(&my_string);
//     println!("first word is: {}", word);
//     let string1 = String::from("hello world");
//     let string2 = String::from("This is a test");
//     let result = longest(&string1, &string2);
//     println!("longest string is {}", result);


//     let char_list= vec!['a', 'n', 'l', 'w', 'e', 'f'];
//     println!("largest char: {}", largest(&char_list));
//     let num_list = vec![1,6,7,4,33,99,7,678,34,54,77];
//     println!("largest num: {}", largest(&num_list));

//     let tweet = Tweet {
//         text: String::from("Hello, World!"),
//         username: String::from("user123"),
//         retweet: true,
//         reply: false
//     };

//     let article = Article {
//         title: String::from("My First Rust Program"),
//         content: String::from("This is my first Rust program"),
//         author: String::from("John Doe")
//     };

//     println!("Summarized Tweet: {}", tweet.summerize());
//     println!("Summerized article: {}", article.summerize());
    

//     let fruits: [&str; 5] = ["apple", "orange", "banana", "pineapple", "guava"];
//     let selected_fruit = fruits[rand::thread_rng().gen_range(0..fruits.len())];

    
//     let mut guess = String::new();


//     while guess.trim().to_lowercase() != selected_fruit {
//         guess.clear();
//         println!("Enter Fruit Name");
//         io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");
//         if guess.trim().to_lowercase() == "exit" {
//             println!("Exiting the game");
//             break;
//         }
//     }

//     println!("You gessed {}", guess.to_lowercase());
// }