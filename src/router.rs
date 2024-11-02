use crate::handlers::*;
use salvo::Router;

pub fn get_router() -> Router {
    let router = Router::new().push(Router::with_path("health_check").get(health_check));
    return router;
}
