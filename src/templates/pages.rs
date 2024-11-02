use askama::Template;
use salvo::http::StatusCode;

#[derive(Template)]
#[template(path = "base.html")]
pub struct Base {}

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")] // using the template in this path, relative
pub struct Index {}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorPage {
    pub error_code: StatusCode,
}
