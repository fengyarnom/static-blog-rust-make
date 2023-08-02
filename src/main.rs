use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::Deserialize;

use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Post {
    title: String,
    date: String,
    tags: Vec<String>,
    categories: Vec<String>,
    content:String
}

fn main() {
    let posts_path = Path::new("./posts");
    if let Ok(entries) = Path::read_dir(posts_path){
        let matter = Matter::<YAML>::new();
        for entry in entries {
            if let Ok(entry) = entry{
                if let Some(extension) = entry.path().extension(){
                    if extension == "md"{
                        let raw_content = fs::read_to_string(entry.path()).unwrap();
                        let result = matter.parse(&raw_content);

                        let frontmatter = result.data.as_ref().unwrap().as_hashmap().unwrap();
                        let markdown_content = result.content;

                        let mut post = Post{
                            title: "".to_string(),
                            date: "".to_string(),
                            tags: vec![],
                            categories: vec![],
                            content: "".to_string(),
                        };

                        for item_frontmatter in &frontmatter {
                            match item_frontmatter.0.as_str() {
                                "title" => {
                                    post.title = item_frontmatter.1.as_string().unwrap();
                                },
                                "date" => {
                                    post.date = item_frontmatter.1.as_string().unwrap();
                                },
                                "tags" => {
                                    if !item_frontmatter.1.is_empty(){
                                        for tag in item_frontmatter.1.as_vec().unwrap(){
                                            post.tags.push(tag.as_string().unwrap())
                                        }
                                    }
                                },

                                "categories" => {
                                    if !item_frontmatter.1.is_empty(){
                                        for category in item_frontmatter.1.as_vec().unwrap(){
                                            post.categories.push(category.as_string().unwrap())
                                        }
                                    }
                                },
                                _ => println!("Unknown frontmatter item"),
                            }

                        }
                        
                        println!("{:?}",post);
                        // if let Ok(front_matter) =  result.data.unwrap().deserialize::<FrontMatter>(){
                        //     println!("{:?}", front_matter);
                        // }else{
                        //     println!("{} missed matter",entry.path().to_string_lossy());
                        // };

                    }
                }
            }

        }
    }else{
        println!("Failed to read the directory.");
    };






    // let matter = Matter::<YAML>::new();
    // let result = matter.parse(INPUT);
    //
    //
    // // Deserialize `result` manually:
    // let front_matter: FrontMatter = result.data.unwrap().deserialize().unwrap();
    // println!("{:?}", front_matter);

}