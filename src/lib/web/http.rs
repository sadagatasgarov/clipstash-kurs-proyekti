use std::fmt::Debug;

use crate::data::AppDatabase;
use crate::service::{self, action};
use crate::web::{ctx, renderer::Renderer, PageError, form};
use crate::{ServiceError, ShortCode};

use rocket::form::{Contextual, Form};
use rocket::http::{Cookie, Status};
use rocket::response::Redirect;
use rocket::response::{content::RawHtml, status};
use rocket::{uri, State};
use rocket::http::CookieJar;

use super::PASSWORD_COOKIE;

#[rocket::get("/")]
fn home(renderer: &State<Renderer<'_>>) -> RawHtml<String> {
    let context = ctx::Home::default();
    RawHtml(renderer.render(context, &[])) 
}

#[rocket::post("/", data="<form>")]
pub async fn new_clip(
    form: Form<Contextual<'_, form::NewClip>>,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,  )
    -> Result<Redirect, (Status, RawHtml<String>)> {
    let form = form.into_inner();
    if let Some(value) = form.value {
        let req = service::ask::NewClip{
            content:value.content,
            title: value.title,
            expires: value.expires,
            password: value.password
        };

        match action::new_clip(req, database.get_pool()).await {
            Ok(clip) => Ok(Redirect::to(uri!(get_clip(shortcode = clip.shortcode)))),
            Err(e) => {
                eprintln!("internet error: {}", e);
                Err((
                    Status::InternalServerError,
                    RawHtml(renderer.render(
                        ctx::Home::default(),
                        &["A server error occured, Please try again"],
                    )),
                ))
            }
        } 
    }  else {
        let errors = form
            .context
            .errors()
            .map(|err| {
                use rocket::form::error::ErrorKind;

                if let ErrorKind::Validation(msg) = &err.kind {
                    msg.as_ref()
                } else {
                    eprintln!("un handled errors: {}", err);
                    "An error occured, please try again"
                }
            })
            .collect::<Vec<_>>();

        Err((
            Status::BadRequest,
            RawHtml(
                renderer.render_with_data(
                    ctx::Home::default(),
                    ("clip", &form.context),
                    &errors
                )
            ),
        ))
    }
}


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



#[rocket::post("/clip/<shortcode>", data="<form>")]
pub async fn submit_clip_password(
    cookies: &CookieJar<'_>,
    form: Form<Contextual<'_, form::GetPasswordProtectedClip>>,
    shortcode: ShortCode,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,  )
    -> Result<RawHtml<String>, PageError> {
    let form = form.into_inner();
    if let Some(form) = &form.value {
        let req = service::ask::GetClip {
            shortcode: shortcode.clone(),
            password: form.password.clone(),
        };
        match action::get_clip(req, database.get_pool()).await {
            Ok(clip) => {
                let context = ctx::ViewClip::new(clip);
                cookies.add(Cookie::new(
                    PASSWORD_COOKIE,
                    form.password.clone().into_inner().unwrap_or_default()                    
                    )
                );
                Ok(RawHtml(renderer.render(context, &[])))
            }
            Err(e) => match e  {
                ServiceError::PermissionError(e) => {
                    let context = ctx::PasswordRequired::new(shortcode);
                    Ok(RawHtml(renderer.render(context, &[e.as_str()])))
                }
                ServiceError::NotFound => Err(PageError::NotFound("Clip not found".to_string())),
                _ => Err(PageError::Internal("server error". to_owned())),
            }
        }
    } else {
        let context = ctx::PasswordRequired::new(shortcode);
        Ok(RawHtml(
            renderer.render(context, &["A password is required to view this clip"])
        ))
    }
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![home, get_clip, submit_clip_password]
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