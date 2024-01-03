mod auth;
mod app;
mod todo;
mod users;
use auth::auth_views_factory;
use todo::to_do_views_factory;
use users::user_views_factory;
use app::app_views_factory;

use actix_web::web::ServiceConfig;

use crate::models::user;

pub fn views_factory(app: &mut ServiceConfig) {
    auth_views_factory(app);
    to_do_views_factory(app);
   app_views_factory(app);
    user_views_factory(app);
}
