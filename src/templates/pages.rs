use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct Base {}

#[derive(Template)] // this will generate the code...
#[template(path = "index.html", print = "all")] // using the template in this path, relative
pub struct Index {}
