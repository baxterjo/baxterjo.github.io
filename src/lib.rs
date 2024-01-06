pub mod app;
pub mod components;
pub mod content;
pub mod errors;
pub mod markdown;
pub mod pages;
pub mod router;

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
