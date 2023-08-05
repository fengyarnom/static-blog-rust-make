use std::fs;
use std::path::PathBuf;
use serde::{Serialize};
use crate::markdown_parse;
use crate::markdown_parse::Post;

#[derive(Serialize, Debug)]
pub struct Posts{
    posts: Vec<Post>
}

impl Posts{
    pub fn new(markdown_file_paths: &Vec<PathBuf>) -> Option<Posts> {
        let mut posts: Vec<Post> = vec![];

        for file_path in markdown_file_paths{
            let raw_content = fs::read_to_string(file_path).unwrap();
            let post = markdown_parse::Post::new(&raw_content).unwrap();
            posts.push(post);
        }

        Some(Posts{posts})

    }

    pub fn get_posts(&self) -> &Vec<Post> {
        &self.posts
    }
}