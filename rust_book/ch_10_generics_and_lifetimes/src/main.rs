use std::fmt::Display;

pub trait Summary {
  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize_author(&self) -> String {
    format!("@{}", self.author)
  }
}

pub struct SocialPost {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub repost: bool,
}

impl Summary for SocialPost {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }

  fn summarize(&self) -> String {
    format!("Post Summary: {}", self.content)
  }
}

#[derive(Debug)]
struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl<T: Display + PartialOrd> Pair<T> {
  // cmp_display is only available if T Implements both Display AND PartialOrd.
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}

fn trait_bound_1() {
  let pair = Pair::new(1, 3);
  pair.cmp_display();
  // println!("Pair: {}", pair)

  #[derive(Debug)]
  struct MyStr {}
  // pair_2 does not have access to cmp_display.
  let pair_2 = Pair::new(MyStr {}, MyStr {});
  dbg!(pair_2);
}

fn generic_lifetimes() {
  /// We want the signature to express the following constraint: The returned reference will be valid as long as both of the parameters are valid.
  /// The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a. In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments. These relationships are what we want Rust to use when analyzing this code.
  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
  }
  let string1 = String::from("abcd");
  let string2 = "xyz";
  let s = longest(string1.as_str(), string2);
  println!("Longest: {}", s);

  let string1 = String::from("long string is long");
  {
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {result}");
  }
}

fn lifetimes_structs() {
  struct ImportantExcerpt<'a> {
    part: &'a str,
  }

  impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
      3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
      println!("Attention please: {announcement}");
      self.part
    }
  }

  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().unwrap();
  let i = ImportantExcerpt { part: first_sentence };
  println!("Exerpt: {}", i.part);
  println!("Excerpt Level: {}", i.level());
  println!("Announce: {}", i.announce_and_return_part("THIS is an announcement"));
}

fn all_in_one() {
  fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: &T) -> &'a str
  where
    T: Display,
  {
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
  }

  struct MyStr {}
  impl Display for MyStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "Hello from MyStr")
    }
  }

  let string1 = String::from("abcd");
  let string2 = "xyz";
  let ann = MyStr {};
  longest_with_an_announcement(string1.as_str(), string2, &ann);
  println!("Ann: {}", ann);
}

fn main() {
  all_in_one();
  lifetimes_structs();
  generic_lifetimes();
  trait_bound_1();

  let post = SocialPost {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    repost: false,
  };

  println!("1 new post: {}", post.summarize());

  let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
      "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
    ),
  };

  notify(&article);
  notify(&post);

  notify_simple(&article);
  notify_simple(&post);

  let article_2 = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Bogota, DT, Colombia"),
    author: String::from("Daniel"),
    content: String::from(
      "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
    ),
  };

  // Have to be of the same type.
  notify_two(&article, &article_2);
  // Don't have to be of the same type.
  notify_two_simple(&article, &post);

  notify(&returns_summarizable());
}

pub fn notify_simple(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

pub fn notify<T>(item: &T)
where
  T: Summary,
{
  println!("Breaking news! {}", item.summarize());
}

pub fn notify_two_simple(item1: &impl Summary, item2: &impl Summary) {
  println!("Breaking news! {}", item1.summarize());
  println!("Breaking news! {}", item2.summarize());
}

pub fn notify_two<T: Summary>(item1: &T, item2: &T) {
  println!("Breaking news! {}", item1.summarize());
  println!("Breaking news! {}", item2.summarize());
}

fn returns_summarizable() -> impl Summary {
  SocialPost {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    repost: false,
  }
}
