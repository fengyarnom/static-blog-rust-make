use std::collections::HashMap;

use crate::markdown_parse::Post;
use crate::posts::Posts;


#[derive(Debug)]
pub struct Categories<'a>{
    categories: HashMap<String, Vec<&'a Post>>,
}

impl Categories<'_> {
    pub fn new<'a>(posts: &'a Posts) -> Option<Categories<'a>> {
        let mut categories: HashMap<String, Vec<&Post>> = HashMap::new();
        for post in posts.get_posts() {
            for category in post.get_categories() {
                categories.entry(category.to_string()).or_insert_with(Vec::new).push(post);
            }
        }
        Some(Categories{categories})
    }
    pub fn get_categories(&self) -> &HashMap<String, Vec<&Post>> {
        &self.categories
    }
}