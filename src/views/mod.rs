mod auth;
mod app;
mod todo;
use auth::auth_views_factory;
use todo::to_do_views_factory;

use actix_web::web::ServiceConfig;

pub fn views_factory(app: &mut ServiceConfig) {
    auth_views_factory(app);
    to_do_views_factory(app);
    app::app_views_factory(app)
}
