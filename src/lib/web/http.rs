use crate::data::AppDatabase;
use crate::service;
use crate::service::action;
use crate::web::{ctx, form, renderer::Renderer, PageError};
use crate::{ServiceError, ShortCode};

use rocket::form::{Contextual, Form};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::{content::RawHtml, status, Redirect};
use rocket::{uri, State};


#[rocket::get("/")]
fn home(renderer: &State<Renderer<'_>>) -> RawHtml<String> {
    let context = ctx::Home::default();
    RawHtml(renderer.render(context, &[])) 
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![home]
}


pub mod catcher {
    use rocket::Request;
    use rocket::{catch, catchers, Catcher};

    #[catch(default)]
    fn default(req: &Request) -> &'static str {
        eprintln!("General error: {:?}", req);
        "something went wrong"
    }

    #[catch(default)]
    fn internal_error(req: &Request) -> &'static str {
        eprintln!("internel error: {:?}", req);
        "Internel server error"
    }

    #[catch(default)]
    fn not_found(req: &Request) -> &'static str {
        eprintln!("404 not foundd: {:?}", req);
        "404"
    }

    pub fn catchers() -> Vec<Catcher> {
        catchers![not_found, default, internal_error]
    }
}