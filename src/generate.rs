use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use chrono::{NaiveDateTime, ParseError};
use comrak::{ComrakOptions, markdown_to_html};
use serde::{Deserialize, Serialize};
use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::ser::SerializeStruct;
use tera::{Context, Tera};

#[derive(Deserialize,Clone, Debug)]
pub struct FrontMatter {
    pub title: String,
    pub date: String,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
}
#[derive( Deserialize,Serialize,Clone, Debug)]
pub struct Post{
    pub title: String,
    pub date: String,
    pub date_simp: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,

    pub raw_content: String,
    pub content: String,

    pub slug: String,
    pub link: String,

}

impl Post {
    pub fn new(
        title: String,
        date: String,
        date_simp: String,
        tags: Vec<String>,
        categories: Vec<String>,
        raw_content: String,
        content: String,
        slug: String,
        link: String,
    ) -> Self {
        Post {
            title,
            date,
            date_simp,
            tags,
            categories,
            raw_content,
            content,
            slug,
            link
        }
    }
    pub fn parse_date_string(date_str: &str) -> Result<NaiveDateTime, ParseError> {
        NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S")
    }
}

#[derive( Deserialize,Serialize,Clone, Debug)]
pub struct Page{
    limited_cows:usize,
    total: usize,
    current: usize,
    prev:usize,
    prev_link: String,
    next:usize,
    next_link: String,
}
impl Page {
    pub fn new(limited_cows: usize, total: usize, current: usize, base_link: &str) -> Page {

        let mut prev = current-1;
        let mut next = current+1;
        let mut prev_link= format!("{}page/{}",base_link,prev);
        let mut next_link= format!("{}page/{}",base_link,next);
        if current == 1 {
            prev_link = format!("./");
        }
        if current == 2 {
            prev_link = format!("{}",base_link);
        }
        if current == total/limited_cows+1 {
            next_link = format!("{}page/{}",base_link,current);
        }
        Page{
            limited_cows,
            total,
            current,
            prev,
            prev_link,
            next,
            next_link,
        }
    }
}

pub struct Global<'a>{
    pub posts: Vec<Post>,
    pub tags: HashMap<String, Vec<&'a Post>>,
    pub categories: HashMap<String, Vec<&'a Post>>,
}


pub fn parse_markdown_file(markdown_content: &str) -> Post {
    let matter = Matter::<YAML>::new();
    let result = matter.parse(markdown_content);

    let mut front_matter: FrontMatter = result.data.unwrap().deserialize().unwrap();
    let raw_content = result.content;

    if front_matter.tags == None{
        front_matter.tags = Some(vec!["post".to_string()]);
    }

    if front_matter.categories == None{
        front_matter.categories = Some(vec!["default".to_string()]);
    }

    let content = markdown_to_html(&raw_content, &ComrakOptions::default());
    let slug:String = format!("{}-{}",front_matter.title,front_matter.date);
    let link: String = format!("/posts/{}.html",slug);
    let parsed_date = Post::parse_date_string(&front_matter.date).unwrap();
    let date_simp = parsed_date.date().to_string();
    Post::new(
        front_matter.title,
        front_matter.date,
        date_simp,
        front_matter.tags.unwrap(),
        front_matter.categories.unwrap(),
        raw_content,content,slug,link)
}

pub struct SiteGenerator {
    content_path: PathBuf,
    templates_path: PathBuf,
    custom_templates_path: PathBuf,
    output_path: PathBuf,
}

impl SiteGenerator {
    pub fn new(content_path: &str, templates_path: &str, custom_templates_path: &str,output_path: &str) -> Self {
        SiteGenerator {
            content_path: PathBuf::from(content_path),
            templates_path: PathBuf::from(templates_path),
            custom_templates_path: PathBuf::from(custom_templates_path),
            output_path: PathBuf::from(output_path),
        }
    }
    pub fn generate_site(&self) -> Result<(), Box<dyn std::error::Error>> {

        let mut global: Global = Global{
            posts: vec![],
            tags: HashMap::new(),
            categories: HashMap::new(),
        };

        // 处理 posts
        // 遍历 content_path / posts 下的 Markdown 文件
        let posts_path = format!("{}/posts",self.content_path.to_string_lossy());
        for entry in fs::read_dir(&posts_path).unwrap() {
            if let Ok(entry) = entry{
                 if entry.path().is_file() {
                     let post = parse_markdown_file(&fs::read_to_string(entry.path()).unwrap());
                     // global 吸纳
                     global.posts.push(post);
                 }
            }
        }
        global.posts.sort_by(|a, b| {
            let date_a = Post::parse_date_string(&a.date).unwrap_or_else(|_| NaiveDateTime::from_timestamp_opt(0, 0).unwrap());
            let date_b = Post::parse_date_string(&b.date).unwrap_or_else(|_| NaiveDateTime::from_timestamp_opt(0, 0).unwrap());

            date_b.cmp(&date_a) // 降序排列
        });

        // 处理 tags categories
        // let mut tags: HashMap<String, Vec<&Post>> = HashMap::new();
        for post in &global.posts{
            for tag in &post.tags{
                global.tags.entry(tag.to_string()).or_insert(Vec::new()).push(&post);
            }

            for category in &post.categories{
                global.categories.entry(category.to_string()).or_insert(Vec::new()).push(&post);
            }
        }

        let scss_input_dir = "sources/static/css";
        let css_output_dir = "public/static/css";
        fs::create_dir_all(css_output_dir).unwrap();
        for entry in fs::read_dir(&scss_input_dir).unwrap() {
            if let Ok(entry) = entry{
                let file_name = entry.file_name().into_string().unwrap();
                if file_name.ends_with(".scss") {
                    let css_content =
                        grass::from_path(entry.path(),&grass::Options::default()).unwrap();
                    fs::write(format!("{}/{}.css",css_output_dir,entry.path().file_stem().unwrap().to_string_lossy()), css_content)?;

                } else if file_name.ends_with(".css") {
                    fs::copy(format!("{}/{}",scss_input_dir,file_name), format!("{}/{}",css_output_dir,file_name)).unwrap();
                }
            }

        }

        let img_input_dir = "sources/static/images";
        let img_output_dir = "public/static/images";
        fs::create_dir_all(img_output_dir).unwrap();

        for entry in fs::read_dir(&img_input_dir).unwrap() {
            if let Ok(entry) = entry{
                let file_name = entry.file_name().into_string().unwrap();
                fs::copy(format!("{}/{}",img_input_dir,file_name), format!("{}/{}",img_output_dir,file_name)).unwrap();

            }

        }


        self.generate_post_page("post.html","public/posts",&global).unwrap();
        self.generate_home_page("index.html","public/",&global).unwrap();
        self.generate_tags_page("tags.html","public/tags",&global).unwrap();
        self.generate_categories_page("categories.html","public/categories",&global).unwrap();
        self.generate_archive_page("archive.html","public/archive",&global).unwrap();
        self.generate_about_page("about.html","public/about",&global).unwrap();
        self.generate_link_page("link.html","public/link",&global).unwrap();

        Ok(())
    }
    fn generate_post_page(&self,template:&str, output_path: &str, global_data: &Global) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(output_path).unwrap();
        let mut tera = Tera::new("./sources/templates/**/*.html").unwrap();
        let mut context = Context::new();

        context.insert("posts",&global_data.posts);
        context.insert("tags",&global_data.tags);
        context.insert("categories",&global_data.categories);

        for post in &global_data.posts {
            context.insert("post",&post);

            let output = format!("{}/{}.html", output_path, &post.slug);
            let rendered = tera.render(template, &context).unwrap();
            fs::write(output, rendered)?;
        }

        Ok(())
    }
    fn generate_home_page(&self,template:&str, output_path: &str, global_data: &Global) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(output_path).unwrap();
        let mut tera = Tera::new("./sources/templates/**/*.html").unwrap();
        let mut context = Context::new();
        let mut output= String::new();

        context.insert("posts",&global_data.posts);
        context.insert("tags",&global_data.tags);
        context.insert("categories",&global_data.categories);

        let limited_cows = 7;
        let mut current = 1;
        let total = global_data.posts.len();

        while current <=  total / limited_cows + 1{
            let page = Page::new(limited_cows,total,current,"/");
            context.insert("posts",&global_data.posts[(current-1)*6..((current*6)-1)]);
            context.insert("page",&page);

            if current == 1{
                output = format!("{}/index.html", output_path);
            }else{
                let output_folder = format!("{}/page/{}",output_path,current);
                fs::create_dir_all(&output_folder).unwrap();
                output = format!("{}/index.html", &output_folder);
            }


            let rendered = tera.render(template, &context).unwrap();
            fs::write(output, rendered)?;
            current += 1;
        }

        Ok(())
    }
    fn generate_tags_page(&self,template:&str, output_path: &str, global_data: &Global) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(output_path).unwrap();

        let mut tera = Tera::new("./sources/templates/**/*.html").unwrap();
        let mut context = Context::new();
        let mut output= String::new();

        context.insert("posts",&global_data.posts);
        context.insert("tags",&global_data.tags);
        context.insert("categories",&global_data.categories);

        let custom_post = parse_markdown_file(&fs::read_to_string("./sources/content/tags/tags.md").unwrap());
        context.insert("post",&custom_post);

        output = format!("public/tags/index.html");
        let rendered = tera.render(template, &context).unwrap();
        fs::write(output, rendered)?;

        for tag in &global_data.tags{
            let output_path = format!("public/tags/{}", tag.0.to_string());
            fs::create_dir_all(output_path).unwrap();

            context.insert("posts", tag.1);
            context.insert("tag_name", tag.0);

            output = format!("public/tags/{}/index.html", tag.0.to_string());
            let rendered = tera.render("archive.html", &context).unwrap();
            fs::write(output, rendered)?;
        }



        Ok(())
    }
    fn generate_categories_page(&self,template:&str, output_path: &str, global_data: &Global) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(output_path).unwrap();

        let mut tera = Tera::new("./sources/templates/**/*.html").unwrap();
        let mut context = Context::new();
        let mut output= String::new();

        context.insert("posts",&global_data.posts);
        context.insert("tags",&global_data.tags);
        context.insert("categories",&global_data.categories);

        let custom_post = parse_markdown_file(&fs::read_to_string("./sources/content/categories/categories.md").unwrap());
        context.insert("post",&custom_post);

        output = format!("public/categories/index.html");
        let rendered = tera.render(template, &context).unwrap();
        fs::write(output, rendered)?;

        for category in &global_data.categories{
            let output_path = format!("public/categories/{}", category.0.to_string());
            fs::create_dir_all(output_path).unwrap();

            context.insert("posts", category.1);
            context.insert("category_name", category.0);

            output = format!("public/categories/{}/index.html", category.0.to_string());
            let rendered = tera.render("archive.html", &context).unwrap();
            fs::write(output, rendered)?;
        }



        Ok(())
    }

    fn generate_archive_page(&self,template:&str, output_path: &str, global_data: &Global) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(output_path).unwrap();

        let mut tera = Tera::new("./sources/templates/**/*.html").unwrap();
        let mut context = Context::new();
        let mut output= String::new();

        context.insert("posts",&global_data.posts);
        context.insert("tags",&global_data.tags);
        context.insert("categories",&global_data.categories);

        let custom_post = parse_markdown_file(&fs::read_to_string("./sources/content/archive/archive.md").unwrap());
        context.insert("post",&custom_post);
        output = format!("{}/index.html", output_path);
        let rendered = tera.render(template, &context).unwrap();
        fs::write(output, rendered)?;
        Ok(())
    }

    fn generate_about_page(&self,template:&str, output_path: &str, global_data: &Global) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(output_path).unwrap();
        let mut tera = Tera::new("./sources/templates/**/*.html").unwrap();
        let mut context = Context::new();
        let mut output= String::new();

        context.insert("posts",&global_data.posts);
        context.insert("tags",&global_data.tags);
        context.insert("categories",&global_data.categories);

        let custom_post = parse_markdown_file(&fs::read_to_string("./sources/content/about/about.md").unwrap());
        context.insert("post",&custom_post);
        output = format!("{}/index.html", output_path);
        let rendered = tera.render(template, &context).unwrap();
        fs::write(output, rendered)?;

        Ok(())
    }
    fn generate_link_page(&self,template:&str, output_path: &str, global_data: &Global) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(output_path).unwrap();
        let mut tera = Tera::new("./sources/templates/**/*.html").unwrap();
        let mut context = Context::new();
        let mut output= String::new();

        context.insert("posts",&global_data.posts);
        context.insert("tags",&global_data.tags);
        context.insert("categories",&global_data.categories);

        let custom_post = parse_markdown_file(&fs::read_to_string("./sources/content/link/link.md").unwrap());
        context.insert("post",&custom_post);
        output = format!("{}/index.html", output_path);
        let rendered = tera.render(template, &context).unwrap();
        fs::write(output, rendered)?;

        Ok(())
    }
    fn generate_page(&self,template:&str, output_path: &str, global_data: &Global) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(output_path).unwrap();

        // 使用模板引擎渲染页面
        // 将文章内容插入模板
        // 生成最终的 HTML 文件并保存到 output_path
        let mut tera = Tera::new("./sources/templates/**/*.html").unwrap();
        let mut context = Context::new();
        let mut output= String::new();

        context.insert("posts",&global_data.posts);
        context.insert("tags",&global_data.tags);
        context.insert("categories",&global_data.categories);


        match template {
            "post.html" => {
                for post in &global_data.posts {
                    context.insert("post",&post);

                    output = format!("{}/{}.html", output_path, &post.slug);
                    let rendered = tera.render(template, &context).unwrap();
                    fs::write(output, rendered)?;
                }
            }
            "index.html" => {
                let limited_cows = 6;
                let mut current = 1;
                let total = global_data.posts.len();

                while current <=  total / limited_cows + 1{
                    let page = Page::new(limited_cows,total,current,"/");
                    context.insert("posts",&global_data.posts[(current-1)*6..((current*6)-1)]);
                    context.insert("page",&page);

                    if current == 1 {
                        output = format!("{}/index.html", output_path);
                    }else{
                        let output_folder = format!("{}/page/{}",output_path,current);
                        fs::create_dir_all(&output_folder).unwrap();
                        output = format!("{}/index.html", &output_folder);
                    }


                    let rendered = tera.render(template, &context).unwrap();
                    fs::write(output, rendered)?;
                    current += 1;
                }

            }
            &_ => {}
        }



        Ok(())
    }


}

