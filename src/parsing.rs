use std::collections::HashMap;
use actix_web::web::post;

use serde::{Serialize};
use gray_matter::{Matter, Pod};
use gray_matter::engine::YAML;
use comrak::{markdown_to_html, ComrakOptions};
use chrono::{NaiveDateTime, ParseError, Timelike};
use crate::file_handling::FileHandler;
#[derive(Serialize, Clone,Debug)]
pub struct Post
{
    pub title: String,
    pub slug: String,
    pub date: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub raw_content: String,
    pub content: String,
    pub link: String,
}

pub fn post_parse(file_handler: &FileHandler
) -> Option<Vec<Post>> {
    let files = &file_handler.markdown_files;
    let mut posts = vec![];

    for file in files{
        // parsing frontmatter and content form markdown file

        let (frontmatter,raw_content)
            = separate_front_matter_and_content(&file.file_content);

        let (title,date,tags,categories) =
            parse_front_matter(&frontmatter).unwrap();

        let (slug,link) = parse_slug_and_link(&title,&date).unwrap();

        let content = parse_markdown_content(&raw_content).unwrap();


        posts.push(Post{title,slug,date,tags,categories,raw_content,content,link});
    }


    posts.sort_by(|a, b| {
        let date_a = parse_date_string(&a.date).unwrap_or_else(|_| NaiveDateTime::from_timestamp(0, 0));
        let date_b = parse_date_string(&b.date).unwrap_or_else(|_| NaiveDateTime::from_timestamp(0, 0));

        date_b.cmp(&date_a) // 降序排列
    });

    Some(posts)
}

fn parse_date_string(date_str: &str) -> Result<NaiveDateTime, ParseError> {
    NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S")
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

fn parse_slug_and_link(title: &str, date: &str) -> Option<(String,String)> {
    let mut slug= String::new();
    let mut link = "/posts/".to_string();

    if let Some((date_part, time_part)) = date.split_once(' ') {
        slug.push_str(date_part);
        slug.push_str("-");
    } else {
        println!("Invalid datetime format");
    }

    slug.push_str(&title.replace(" ", "-"));

    link.push_str(&slug);
    link.push_str(".html");

    Some((slug,link))

}

pub fn parse_markdown_content(raw_content: &String) -> Option<String> {
    let html_content = markdown_to_html(raw_content, &ComrakOptions::default());
    Some(html_content)
}
