use askama::Template;
use salvo::{handler, http::StatusCode, writing::Text, Response};

use crate::templates::Index;

#[handler]
pub fn render_index(res: &mut Response) {
    let index = Index {};

    if let Ok(html_string) = index.render() {
        res.render(Text::Html(html_string));
    } else {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    }
}
