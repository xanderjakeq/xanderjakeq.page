use crate::handlers::*;
use salvo::serve_static::StaticDir;
use salvo::Router;

pub fn get_router() -> Router {
    let router = Router::new()
        .get(render_index)
        .push(Router::with_path("static/<**path>").get(StaticDir::new(["static/"]).auto_list(true)))
        .push(Router::with_path("health_check").get(health_check));

    return router;
}
