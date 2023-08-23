//
// 负责读取博客文章的Markdown文件、处理文件路径、以及创建目标文件夹用于存放生成的静态网站文件。
use std::fs;
use std::path::{PathBuf};
use actix_web::dev::ResourcePath;

#[derive(Debug)]
pub struct FileHandler
{
    // path: String, content: String
    pub markdown_files: Vec<FileContent>,
    pub template_files: Vec<FileContent>,
    pub scss_files:Vec<FileContent>
}

#[derive(Debug)]
pub struct FileContent
{
    pub file_path: String,
    pub file_content: String,
}


impl FileHandler {
    pub fn new() -> Option<FileHandler>{

        FileHandler::create_target_folder();

        let markdown_files =
            FileHandler::
            traverse_files_with_extension("./sources/posts","md")
                .unwrap();


        let template_files =
            FileHandler::
            traverse_files_with_extension("./sources/templates","html")
            .unwrap();

        let scss_files =
            FileHandler::
            traverse_files_with_extension("./sources/static/css","scss")
                .unwrap();

        Some(FileHandler{markdown_files,template_files,scss_files})

    }

    pub fn create_target_folder(){
        let public_path = PathBuf::from("public");

        let posts_path = public_path.join("posts");
        let tags_path = public_path.join("tags");
        let categories_path = public_path.join("categories");
        let static_path = public_path.join("static");

        let css_path = static_path.join("css");


        // 检测并创建public文件夹
        if !fs::metadata(&public_path).is_ok() {
            if let Err(err) = fs::create_dir(&public_path) {
                eprintln!("Error creating 'public' folder: {}", err);
                return;
            }
        }

        // 检测并创建public/posts文件夹
        if !fs::metadata(&posts_path).is_ok() {
            if let Err(err) = fs::create_dir(&posts_path) {
                eprintln!("Error creating 'public/posts' folder: {}", err);
            }
        }

        // 检测并创建public/tags文件夹
        if !fs::metadata(&tags_path).is_ok() {
            if let Err(err) = fs::create_dir(&tags_path) {
                eprintln!("Error creating 'public/tags' folder: {}", err);
            }
        }

        // 检测并创建public/categories文件夹
        if !fs::metadata(&categories_path).is_ok() {
            if let Err(err) = fs::create_dir(&categories_path) {
                eprintln!("Error creating 'public/categories' folder: {}", err);
            }
        }

        // 检测并创建public/static文件夹
        if !fs::metadata(&static_path).is_ok() {
            if let Err(err) = fs::create_dir(&static_path) {
                eprintln!("Error creating 'public/static' folder: {}", err);
            }
        }

        // 检测并创建public/static/css文件夹
        if !fs::metadata(&css_path).is_ok() {
            if let Err(err) = fs::create_dir(&css_path) {
                eprintln!("Error creating 'public/static/css' folder: {}", err);
            }
        }
    }

    pub fn traverse_files_with_extension(folder_path: &str, extension: &str)-> Result<Vec<FileContent>, Box<dyn std::error::Error>>{
        let mut result = Vec::new();
        let folder_path = PathBuf::from(folder_path);

        if folder_path.is_dir() {
            for entry in fs::read_dir(folder_path)? {
                let entry = entry?;
                let file_path = entry.path();

                if file_path.is_file() {
                    if let Some(ext) = file_path.extension() {
                        if ext == extension {
                            let file_path = file_path.to_string_lossy().to_string();
                            let file_content = fs::read_to_string(&file_path).unwrap();

                            result.push(FileContent{file_path,file_content});

                        }
                    }
                }
            }
        }
        Ok(result)
    }
}