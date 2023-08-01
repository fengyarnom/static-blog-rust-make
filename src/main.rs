use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct FrontMatter {
    title: String,
    tags: Vec<String>
}

const INPUT: &str = r#"---
title: gray-matter-rs
tags:
  - gray-matter
  - rust
---
Some excerpt
---
Other stuff
"#;

fn main() {
    let matter = Matter::<YAML>::new();
    let result = matter.parse(INPUT);


    // Deserialize `result` manually:
    let front_matter: FrontMatter = result.data.unwrap().deserialize().unwrap();
    println!("{:?}", front_matter);

}