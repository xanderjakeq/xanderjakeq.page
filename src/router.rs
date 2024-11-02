use crate::handlers::*;
use salvo::catcher::Catcher;
use salvo::serve_static::StaticDir;
use salvo::{Router, Service};

use crate::handlers::render_error_page;

pub fn get_router() -> Service {
    let router = Router::new()
        .get(render_index)
        .push(Router::with_path("static/<**path>").get(StaticDir::new(["static/"]).auto_list(true)))
        .push(Router::with_path("health_check").get(health_check));

    return Service::new(router).catcher(Catcher::default().hoop(render_error_page));
}
