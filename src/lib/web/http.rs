use std::fmt::Debug;

use crate::data::AppDatabase;
use crate::domain::clip;
use crate::domain::clip::field::Password;
use crate::service::{self, action, ask};
use crate::web::{ctx, form, renderer::Renderer, PageError};
use crate::{ServiceError, ShortCode};

use rocket::form::{Contextual, Form};
use rocket::http::CookieJar;
use rocket::http::{Cookie, Status};
use rocket::response::Redirect;
use rocket::response::{content::RawHtml, status};
use rocket::{uri, State};

use super::hitcounter::HitCounter;
use super::PASSWORD_COOKIE;

/// Route to the home page.
#[rocket::get("/")]
fn home(renderer: &State<Renderer<'_>>) -> RawHtml<String> {
    let context = ctx::Home::default();
    RawHtml(renderer.render(context, &[]))
}

/// Route to submit a new [`Clip`](crate::Clip).
#[rocket::post("/", data = "<form>")]
pub async fn new_clip(
    form: Form<Contextual<'_, form::NewClip>>,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,
) -> Result<Redirect, (Status, RawHtml<String>)> {
    let form = form.into_inner();
    if let Some(value) = form.value {
        let req = service::ask::NewClip {
            content: value.content,
            title: value.title,
            expires: value.expires,
            password: value.password,
        };
        match action::new_clip(req, database.get_pool()).await {
            Ok(clip) => Ok(Redirect::to(uri!(get_clip(shortcode = clip.shortcode)))),
            Err(e) => {
                eprintln!("internal error: {}", e);
                Err((
                    Status::InternalServerError,
                    RawHtml(renderer.render(
                        ctx::Home::default(),
                        &["A server error occurred. Please try again"],
                    )),
                ))
            }
        }
    } else {
        let errors = form
            .context
            .errors()
            .map(|err| {
                use rocket::form::error::ErrorKind;
                if let ErrorKind::Validation(msg) = &err.kind {
                    msg.as_ref()
                } else {
                    eprintln!("unhandled error: {}", err);
                    "An error occurred, please try again"
                }
            })
            .collect::<Vec<_>>();
        Err((
            Status::BadRequest,
            RawHtml(renderer.render_with_data(
                ctx::Home::default(),
                ("clip", &form.context),
                &errors,
            )),
        ))
    }
}

/// Route to get a [`Clip`](crate::Clip).
#[rocket::get("/clip/<shortcode>")]
pub async fn get_clip(
    shortcode: ShortCode,
    database: &State<AppDatabase>,
    hit_counter: &State<HitCounter>,
    renderer: &State<Renderer<'_>>,
) -> Result<status::Custom<RawHtml<String>>, PageError> {
    fn render_with_status<T: ctx::PageContext + serde::Serialize + std::fmt::Debug>(
        status: Status,
        context: T,
        renderer: &Renderer,
    ) -> Result<status::Custom<RawHtml<String>>, PageError> {
        Ok(status::Custom(
            status,
            RawHtml(renderer.render(context, &[])),
        ))
    }
    match action::get_clip(shortcode.clone().into(), database.get_pool()).await {
        Ok(clip) => {
            hit_counter.hit(shortcode.clone(), 1);
            let context = ctx::ViewClip::new(clip);
            render_with_status(Status::Ok, context, renderer)
        }
        Err(e) => match e {
            ServiceError::PermissionError(_) => {
                let context = ctx::PasswordRequired::new(shortcode);
                render_with_status(Status::Unauthorized, context, renderer)
            }
            ServiceError::NotFound => Err(PageError::NotFound("Clip not found".to_owned())),
            _ => Err(PageError::Internal("server error".to_owned())),
        },
    }
}

/// Route to submit a [`Password`](crate::domain::clip::field::Password) for a password-protected [`Clip`](crate::Clip).
#[rocket::post("/clip/<shortcode>", data = "<form>")]
pub async fn submit_clip_password(
    cookies: &CookieJar<'_>,
    form: Form<Contextual<'_, form::GetPasswordProtectedClip>>,
    shortcode: ShortCode,
    hit_counter: &State<HitCounter>,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,
) -> Result<RawHtml<String>, PageError> {
    if let Some(form) = &form.value {
        let req = service::ask::GetClip {
            shortcode: shortcode.clone(),
            password: form.password.clone(),
        };
        match action::get_clip(req, database.get_pool()).await {
            Ok(clip) => {
                hit_counter.hit(shortcode.clone(), 1);
                let context = ctx::ViewClip::new(clip);
                cookies.add(Cookie::new(
                    PASSWORD_COOKIE,
                    form.password.clone().into_inner().unwrap_or_default(),
                ));
                Ok(RawHtml(renderer.render(context, &[])))
            }
            Err(e) => match e {
                ServiceError::PermissionError(e) => {
                    let context = ctx::PasswordRequired::new(shortcode);
                    Ok(RawHtml(renderer.render(context, &[e.as_str()])))
                }
                ServiceError::NotFound => Err(PageError::NotFound("Clip not found".to_owned())),
                _ => Err(PageError::Internal("server error".to_owned())),
            },
        }
    } else {
        let context = ctx::PasswordRequired::new(shortcode);
        Ok(RawHtml(renderer.render(
            context,
            &["A password is required to view this clip"],
        )))
    }
}

/// Route to get just the [`Content`](crate::domain::clip::field::Content) of a [`Clip`](crate::Clip).
#[rocket::get("/clip/raw/<shortcode>")]
pub async fn get_raw_clip(
    cookies: &CookieJar<'_>,
    shortcode: ShortCode,
    hit_counter: &State<HitCounter>,
    database: &State<AppDatabase>,
) -> Result<status::Custom<String>, Status> {
    use crate::domain::clip::field::Password;
    let req = service::ask::GetClip {
        shortcode: shortcode.clone(),
        password: cookies
            .get(PASSWORD_COOKIE)
            .map(|cookie| cookie.value())
            .map(|raw_password| Password::new(raw_password.to_string()).ok())
            .flatten()
            .unwrap_or_else(Password::default),
    };
    match action::get_clip(req, database.get_pool()).await {
        Ok(clip) => {
            hit_counter.hit(shortcode.clone(), 1);
            Ok(status::Custom(Status::Ok, clip.content.into_inner()))
        }
        Err(e) => match e {
            ServiceError::PermissionError(msg) => Ok(status::Custom(Status::Unauthorized, msg)),
            ServiceError::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        },
    }
}

/// The URI [`routes`](rocket::Route) which can be mounted by [`rocket`].
pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![home, get_clip, new_clip, submit_clip_password, get_raw_clip]
}

pub mod catcher {
    //! Contains all the page catchers.
    use rocket::Request;
    use rocket::{catch, catchers, Catcher};

    /// Catch unhandled errors.
    #[catch(default)]
    fn default(req: &Request) -> &'static str {
        eprintln!("General error: {:?}", req);
        "something went wrong..."
    }

    /// Catch server errors.
    #[catch(500)]
    fn internal_error(req: &Request) -> &'static str {
        eprintln!("Internal error: {:?}", req);
        "internal server error"
    }

    /// Catch missing data errors.
    #[catch(404)]
    fn not_found() -> &'static str {
        "404"
    }

    /// The [`catchers`](rocket::Catcher) which can be registered by [`rocket`].
    pub fn catchers() -> Vec<Catcher> {
        catchers![not_found, default, internal_error]
    }
}
