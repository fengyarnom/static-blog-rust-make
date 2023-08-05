use std::collections::HashMap;
use serde::__private::de::TagOrContentField::Tag;
use crate::markdown_parse::Post;
use crate::posts::Posts;

#[derive(Debug)]
pub struct Tags<'a>{
    tags: HashMap<String, Vec<&'a Post>>,
}

impl Tags<'_> {
    pub fn new<'a>(posts: &'a Posts) -> Option<Tags<'a>> {
        let mut tags: HashMap<String, Vec<&Post>> = HashMap::new();
        for post in posts.get_posts() {
            for tag in post.get_tags() {
                tags.entry(tag.to_string()).or_insert_with(Vec::new).push(post);
            }
        }
        Some(Tags{tags})
    }
    pub fn get_tags(&self) -> &HashMap<String, Vec<&Post>> {
        &self.tags
    }
}