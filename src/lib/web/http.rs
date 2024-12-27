use std::fmt::Debug;

use crate::data::AppDatabase;
use crate::service::action;
use crate::web::{ctx, renderer::Renderer, PageError};
use crate::{ServiceError, ShortCode};

use rocket::http::Status;
use rocket::response::{content::RawHtml, status};
use rocket::State;


#[rocket::get("/")]
fn home(renderer: &State<Renderer<'_>>) -> RawHtml<String> {
    let context = ctx::Home::default();
    RawHtml(renderer.render(context, &[])) 
}

// #[rocket::post("/", data="<form>")]
// pub async fn new_clip(
    
// )


#[rocket::get("/clip/<shortcode>")]
pub async fn get_clip(
    shortcode: ShortCode,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>
) -> Result<status::Custom<RawHtml<String>>, PageError> {
fn render_with_status<T: ctx::PageContext + serde::Serialize + Debug>(
    status: Status,
    context: T,
    renderer: &Renderer
) -> Result<status::Custom<RawHtml<String>>, PageError> {
    Ok(status::Custom(status, RawHtml(renderer.render(context, &[]))))
}
    
    match action::get_clip(shortcode.clone().into(), database.get_pool()).await {
        Ok(clip) => {
            let context = ctx::ViewClip::new(clip);
            render_with_status(Status::Ok, context, renderer)
        }
        Err(e)=>match e {
            ServiceError::PermissionError(_) => {
                let context = ctx::PasswordRequired::new(shortcode);
                render_with_status(Status::Unauthorized, context, renderer)
            }
            ServiceError::NotFound=>Err(PageError::NotFound("clip not found".to_string())),
            _ => Err(PageError::Internal("Server error".to_owned()))

        }
    }
}


pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![home, get_clip]
}




pub mod catcher {
    use rocket::Request;
    use rocket::{catch, catchers, Catcher};

    #[catch(default)]
    fn default(req: &Request) -> &'static str {
        eprintln!("General error: {:?}", req);
        "something went wrong"
    }

    #[catch(500)]
    fn internal_error(req: &Request) -> &'static str {
        eprintln!("internel error: {:?}", req);
        "Internel server error"
    }

    #[catch(404)]
    fn not_found(req: &Request) -> &'static str {
        eprintln!("404 not foundd: {:?}", req);
        "404"
    }

    pub fn catchers() -> Vec<Catcher> {
        catchers![not_found, default, internal_error]
    }
}