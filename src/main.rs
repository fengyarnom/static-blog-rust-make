mod markdown_parse;
mod traverse_files;
mod render_html;
mod posts;

use std::io::Write;


fn main() {
    let blog_files = traverse_files::BlogFiles::new("./sources").unwrap();
    let posts= posts::Posts::new(blog_files.get_markdown_file_paths()).unwrap();
    let html_render = render_html::RenderHtml::new("./sources/templates/**/*.html", &posts, &blog_files).unwrap();
    html_render.render_html("./output");
}