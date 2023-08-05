use std::collections::HashMap;
use serde::{Serialize};
use gray_matter::{Matter, Pod};
use gray_matter::engine::YAML;
use comrak::{markdown_to_html, ComrakOptions};

#[derive(Serialize, Debug)]
pub struct Post
{
    title: String,
    slug: String,
    date: String,
    tags: Vec<String>,
    categories: Vec<String>,
    content:String
}

impl Post {
    pub fn new(markdown_raw_content:&str) -> Option<Post> {
        let (frontmatter,markdown_content) =  Post::separate_front_matter_and_content(markdown_raw_content);
        let (title,date,tags,categories) = Post::parse_front_matter(&frontmatter).unwrap();
        let content = Post::parse_markdown_to_html(&markdown_content).unwrap();
        let slug = Post::parse_slug(&title,&date);

        Some(Post{title,slug,date,tags,categories,content})
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_date(&self) -> &String {
        &self.date
    }

    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }
    pub fn get_categories(&self) -> &Vec<String> {
        &self.categories
    }
    pub fn get_content(&self) -> &String {
        &self.content
    }
    pub fn get_slug(&self) -> &String {
        &self.slug
    }

    fn separate_front_matter_and_content(markdown_raw_content:&str) -> (HashMap<String, Pod>, String) {
        let matter = Matter::<YAML>::new();
        let parsed_entity = matter.parse(markdown_raw_content);

        let frontmatter = parsed_entity.data.as_ref().unwrap().as_hashmap().unwrap();
        let markdown_content = parsed_entity.content;

        (frontmatter,markdown_content)
    }

    fn parse_front_matter(frontmatter: &HashMap<String,Pod>) -> Option<(String,String,Vec<String>,Vec<String>)> {
        let mut title: String = String::new();
        let mut date: String = String::new();
        let mut tags: Vec<String> = Vec::new();
        let mut categories: Vec<String> = Vec::new();

        for item_frontmatter in frontmatter{
            match item_frontmatter.0.as_str() {
                "title" => {
                    title = item_frontmatter.1.as_string().unwrap();
                },
                "date" => {
                    date = item_frontmatter.1.as_string().unwrap();
                },
                "tags" => {
                    if !item_frontmatter.1.is_empty(){
                        for tag in item_frontmatter.1.as_vec().unwrap(){
                            tags.push(tag.as_string().unwrap())
                        }
                    }
                },

                "categories" => {
                    if !item_frontmatter.1.is_empty(){
                        for category in item_frontmatter.1.as_vec().unwrap(){
                            categories.push(category.as_string().unwrap())
                        }
                    }
                },
                _ => {
                    println!("Unknown frontmatter item");
                    return None
                },
            }
        }

        Some((title,date,tags,categories))
    }

    fn parse_markdown_to_html(markdown_content: &str) -> Option<String>{
        let html_content = markdown_to_html(&markdown_content, &ComrakOptions::default());
        Some(html_content)
    }

    fn parse_slug(title: &str, date: &str) -> String {
        let mut slug= String::new();

        if let Some((date_part, time_part)) = date.split_once(' ') {
            slug.push_str(date_part);
            slug.push_str("-");
        } else {
            println!("Invalid datetime format");
        }

        slug.push_str(&title.replace(" ", "-"));

        slug

    }
}