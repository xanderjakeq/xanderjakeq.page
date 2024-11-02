use askama::Template;
use salvo::{handler, http::StatusCode, writing::Text, Response};

use salvo::FlowCtrl;

use crate::templates::ErrorPage;

#[handler]
pub fn render_error_page(res: &mut Response, ctrl: &mut FlowCtrl) {
    let status_code = res.status_code.unwrap_or(StatusCode::NOT_FOUND);

    let error_page = ErrorPage {
        error_code: status_code,
    };

    if let Ok(html_string) = error_page.render() {
        res.render(Text::Html(html_string));
    }
    ctrl.skip_rest();
}
