mod markdown_parse;
mod traverse_files;
mod render_html;
mod posts;
mod tags;
mod categories;

use std::io::Write;
use crate::categories::Categories;
use crate::tags::Tags;


fn main() {
    let blog_files = traverse_files::BlogFiles::new("./sources").unwrap();
    let posts= posts::Posts::new(blog_files.get_markdown_file_paths()).unwrap();
    let tags = Tags::new(&posts).unwrap();
    let categories = Categories::new(&posts).unwrap();
    let html_render = render_html::RenderHtml::new("./sources/templates/**/*.html", &posts, &blog_files,&tags,&categories).unwrap();
    html_render.render_html("./output");
}