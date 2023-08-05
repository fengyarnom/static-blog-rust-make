use std::fs;
use std::path::{PathBuf};

#[derive(Debug)]
pub struct BlogFiles{
    markdown_file_paths: Vec<PathBuf>,
    template_file_paths: Vec<PathBuf>,
    scss_file_paths: Vec<PathBuf>,
}

impl BlogFiles {
    pub fn new(sources_path: &str) -> Option<BlogFiles>{
        let sources_path = PathBuf::from(sources_path);
        let markdown_file_paths = BlogFiles::traverse_markdown_files(&sources_path).unwrap();
        let template_file_paths = BlogFiles::traverse_templates_files(&sources_path).unwrap();
        let scss_file_paths = BlogFiles::traverse_scss_files(&sources_path).unwrap();
        Some(BlogFiles{markdown_file_paths,template_file_paths,scss_file_paths})
    }

    pub fn get_template_file_paths(&self) -> &Vec<PathBuf>{
        &self.template_file_paths
    }

    pub fn get_markdown_file_paths(&self) -> &Vec<PathBuf>{
        &self.markdown_file_paths
    }

    pub fn get_scss_file_paths(&self) -> &Vec<PathBuf>{
        &self.scss_file_paths
    }
    fn traverse_markdown_files(sources_path: &PathBuf)-> Option<Vec<PathBuf>>{
        let posts_path = "posts";
        let posts_path = sources_path.join(posts_path);

        let mut markdown_file_paths: Vec<PathBuf> = vec![];
        if let Ok(entries) = fs::read_dir(posts_path){
            for entry in entries{
                if let Ok(post) = entry{
                    let post_path = post.path();

                    markdown_file_paths.push(post_path);

                }
            }
        }
        Some(markdown_file_paths)
    }

    fn traverse_scss_files(sources_path: &PathBuf)-> Option<Vec<PathBuf>>{
        let css_path = "static/css";
        let css_path = sources_path.join(css_path);

        let mut scss_file_paths: Vec<PathBuf> = vec![];
        if let Ok(entries) = fs::read_dir(css_path){
            for entry in entries{
                if let Ok(post) = entry{
                    let css_path = post.path();

                    scss_file_paths.push(css_path);

                }
            }
        }
        Some(scss_file_paths)
    }

    fn traverse_templates_files(sources_path: &PathBuf)-> Option<Vec<PathBuf>>{
        let templates_path = "templates";
        let templates_path = sources_path.join(templates_path);

        let mut template_file_paths: Vec<PathBuf> = vec![];
        if let Ok(entries) = fs::read_dir(templates_path){
            for entry in entries{
                if let Ok(template_file) = entry{
                    let template_file_path = template_file.path();

                    template_file_paths.push(template_file_path);

                }
            }
        }
        Some(template_file_paths)
    }
}