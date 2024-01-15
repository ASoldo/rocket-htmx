// Enable experimetal features for Rocket
#![feature(proc_macro_hygiene, decl_macro)]

// Import the Rocket and Serde crates, including their macros
#[macro_use]
extern crate rocket;
// use rocket::futures::lock::Mutex;
use rocket_contrib::templates::Template;
#[macro_use]
extern crate serde;

// Import necessary modules and types
use reqwest;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::get;
use rocket::http::{Cookie, CookieJar, Header};
use rocket::response::content;
use rocket::response::stream::EventStream;
use rocket::serde::json::Json;
use rocket::tokio::time::{interval, Duration};
use rocket::{response::stream::Event, State};
use rocket::{Request, Response};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tera::Context;
use tera::Tera;
use uuid::Uuid;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

// #[get("/events")]
// fn events() -> EventStream![] {
//     let mut interval = interval(Duration::from_secs(1));
//     EventStream! {
//         interval.tick().await;
//      yield Event::data("Hello, world!".to_string()).id("1");
//     }
// }

// #[get("/events")]
// fn events() -> EventStream![] {
//     let mut interval = interval(Duration::from_secs(1));
//     EventStream! {
//     loop {
//         interval.tick().await;
//         let message = format!(r#"<div>Ojla from events</div>"#);
//
//          yield Event::data(message).id("1");
//         }
//     }
// }

#[get("/events")]
fn events() -> EventStream![] {
    let mut interval = interval(Duration::from_secs(1));
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            panic!("Problem setting up Tera: {:?}", e);
        }
    };
    EventStream! {
        loop {
            interval.tick().await;
            let mut context = Context::new();
            context.insert("name", "Andrija");
            let mut message = tera
                .render("sse-comp.html", &context)
                .expect("Failed to render template.");
            message = message.replace("\n", "");
            yield Event::data(message).id("1");
        }
    }
}

// #[get("/events")]
// fn events() -> EventStream![] {
//     let mut interval = interval(Duration::from_secs(1));
//     EventStream! {
//     loop {
//         interval.tick().await;
//
//          yield Event::data("Hello, world!".to_string()).id("1");
//         }
//     }
// }

#[get("/hello/<name>")]
fn hello(name: String) -> content::RawHtml<String> {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            panic!("Problem setting up Tera: {:?}", e);
        }
    };

    let mut context = Context::new();
    context.insert("name", &name);

    let rendered = tera
        .render("index-demo.html", &context)
        .expect("Failed to render template.");

    content::RawHtml(rendered)
}

#[get("/")]
fn get_index() -> content::RawHtml<String> {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            panic!("Problem setting up Tera: {:?}", e);
        }
    };

    let context = Context::new();

    let rendered = tera
        .render("index.html", &context)
        .expect("Failed to render template.");

    content::RawHtml(rendered)
}

struct Counter {
    count: Mutex<i32>,
}

#[get("/increment")]
fn get_comp(counter: &State<Counter>) -> content::RawHtml<String> {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            panic!("Problem setting up Tera: {:?}", e);
        }
    };

    let name = "Andrej";
    let last_name = "Soldo";
    let mut counter = counter.count.lock().unwrap();
    *counter += 1;

    let mut context = Context::new();
    context.insert("name", &name);
    context.insert("last_name", &last_name);
    context.insert("counter", &*counter);

    let rendered = tera
        .render("comp.html", &context)
        .expect("Failed to render template.");

    content::RawHtml(rendered)
}

#[get("/increment-user")]
fn get_comp_user(cookies: &CookieJar<'_>) -> content::RawHtml<String> {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            panic!("Problem setting up Tera: {:?}", e);
        }
    };

    let mut user_count = cookies
        .get("user_counter")
        .and_then(|cookie| cookie.value().parse::<i32>().ok())
        .unwrap_or(0);
    user_count += 1;

    cookies.add(Cookie::new("user_counter", user_count.to_string()));

    let name = "Andrej";
    let last_name = "Soldo";

    let mut context = Context::new();
    context.insert("name", &name);
    context.insert("last_name", &last_name);
    context.insert("user_counter", &user_count);

    let rendered = tera
        .render("comp-user.html", &context)
        .expect("Failed to render template.");

    content::RawHtml(rendered)
}

#[get("/comp1")]
fn get_comp1() -> content::RawHtml<String> {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            panic!("Problem setting up Tera: {:?}", e);
        }
    };

    let name = "Andrej";
    let last_name = "Soldo";

    let mut context = Context::new();
    context.insert("name", &name);
    context.insert("last_name", &last_name);

    let rendered = tera
        .render("comp1.html", &context)
        .expect("Failed to render template.");

    content::RawHtml(rendered)
}

#[get("/clock")]
fn get_clock() -> content::RawHtml<String> {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            panic!("Problem setting up Tera: {:?}", e);
        }
    };

    let mut context = Context::new();

    let rendered = tera
        .render("clock.html", &context)
        .expect("Failed to render template.");

    content::RawHtml(rendered)
}

#[derive(Serialize, Deserialize, Debug)]
struct Pokemon {
    name: String,
    id: i32,
    sprites: Sprites,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sprites {
    front_default: String,
}

#[get("/pokemon/<id>")]
async fn get_pokemon(id: i32) -> Json<Pokemon> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", id);
    let response: Pokemon = reqwest::get(&url).await.unwrap().json().await.unwrap();

    Json(response)
}

// Define the Post struct with Serde derives for (de)serialization
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Post {
    id: String,
    title: String,
    content: String,
}

// Define a type alias for a thread-safe HashMap to store blog posts
type BlogPosts = Arc<Mutex<HashMap<String, Post>>>;

// Define the GET /posts route to retrieve all blog posts
#[get("/posts")]
fn get_posts(blog_posts: &State<BlogPosts>) -> Json<Vec<Post>> {
    // Lock the blog_posts HashMap, clone the values, and collect them into a Vec
    let posts = blog_posts.lock().unwrap().values().cloned().collect();
    // Return the posts as JSON
    Json(posts)
}

// Define the POST /posts route to create a new blog post
#[post("/posts", format = "json", data = "<post>")]
fn create_post(post: Json<Post>, blog_posts: &State<BlogPosts>) -> Json<Post> {
    // Convert the JSON data into a Post struct
    let mut new_post = post.into_inner();
    // Generate a new UUID as the post ID
    new_post.id = Uuid::new_v4().to_string();
    // Insert the new post into the blog_posts HashMap
    blog_posts
        .lock()
        .unwrap()
        .insert(new_post.id.clone(), new_post.clone());
    // Return the created post as JSON
    Json(new_post)
}

#[derive(Debug)]
enum Soldo {
    A,
    B,
    C,
}

#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T> BinaryTree<T> {
    fn add(&mut self, value: T)
    where
        T: Ord,
    {
        match self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::NonEmpty(node) => {
                if value < node.element {
                    node.left.add(value);
                } else if value > node.element {
                    node.right.add(value);
                }
            }
        }
    }

    fn print(&self)
    where
        T: std::fmt::Debug,
    {
        match self {
            BinaryTree::Empty => (),
            BinaryTree::NonEmpty(node) => {
                node.left.print();
                println!("{:?}", node.element);
                node.right.print();
            }
        }
    }

    fn contains(&self, value: T) -> bool
    where
        T: Ord,
    {
        match self {
            BinaryTree::Empty => false,
            BinaryTree::NonEmpty(node) => {
                if value < node.element {
                    node.left.contains(value)
                } else if value > node.element {
                    node.right.contains(value)
                } else {
                    true // value is equal to node.element
                }
            }
        }
    }
}

fn word_to_number(word: &str) -> u32 {
    word.bytes().map(u32::from).sum()
}

// Define the main function that launches the Rocket server
#[launch]
fn rocket() -> _ {
    let neki: Soldo = Soldo::A;
    let drugi: Pokemon = Pokemon {
        name: String::from("Soldo"),
        id: 0,
        sprites: Sprites {
            front_default: String::from("htt"),
        },
    };
    let treci: Soldo = Soldo::C;

    match neki {
        Soldo::A => {
            println!(" It's A, {:?}", neki)
        }
        Soldo::B => {}
        Soldo::C => {}
    }

    let mut tree = BinaryTree::Empty;
    tree.add(1);
    tree.add(4);
    tree.add(11);
    tree.add(8);
    tree.add(5);
    tree.add(2);
    tree.add(6);
    tree.add(10);
    tree.add(7);
    tree.add(9);
    tree.add(3);
    tree.add(12);
    tree.print();

    println!("Does it contain 12: {:?}", tree.contains(12));
    println!("Does it contain 200: {:?}", tree.contains(200));
    println!("Does it contain 2: {:?}", tree.contains(2));

    let text = "hello world this is a test";
    let words: Vec<_> = text.split_whitespace().collect();

    let mut tree = BinaryTree::Empty;
    for word in &words {
        tree.add(word_to_number(word));
    }

    let query_word = "test";
    let query_number = word_to_number(query_word);
    if tree.contains(query_number) {
        println!("The tree contains {}!", query_word)
    } else {
        println!("The tree does not contain {}!", query_word);
    }

    // Create the BlogPosts instance with an empty HashMap
    let blog_posts: BlogPosts = Arc::new(Mutex::new(HashMap::new()));
    let example_post_id = Uuid::new_v4().to_string();
    // Add an example post to the blog_posts HashMap
    blog_posts.lock().unwrap().insert(
        example_post_id.clone(),
        Post {
            id: example_post_id,
            title: "Hello, world!".to_string(),
            content: "This is my first rust blog post API 🦀.".to_string(),
        },
    );

    // Build and launch the Rocket server with the blog_posts instance and routes
    rocket::build()
        .attach(CORS)
        .manage(blog_posts)
        .manage(Counter {
            count: Mutex::new(0),
        })
        .mount(
            "/",
            routes![
                get_posts,
                create_post,
                get_pokemon,
                hello,
                get_index,
                get_comp1,
                get_comp,
                get_comp_user,
                get_clock,
                events
            ],
        )
}

#[cfg(test)]
mod tests {
    // use super::*;
}
