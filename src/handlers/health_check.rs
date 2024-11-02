use salvo::{handler, http::StatusCode, Response};

#[handler]
pub async fn health_check(res: &mut Response) {
    res.status_code(StatusCode::OK);
}
