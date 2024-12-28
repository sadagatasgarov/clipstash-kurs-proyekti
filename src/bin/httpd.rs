use clipstash::data::AppDatabase;
use clipstash::web::renderer::Renderer;
use dotenv::dotenv;
use std::path::PathBuf;

use structopt::StructOpt;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions, Cors};

#[derive(StructOpt, Debug)]
#[structopt(name = "httpd")]
struct Opt {
    #[structopt(default_value = "sqlite:data.db")]
    connection_string: String,
    #[structopt(short, long, parse(from_os_str), default_value="templates/")]
    template_directory: PathBuf
}

fn main(){
    dotenv().ok();
    let opt = Opt::from_args();
 // CORS seçeneklerini yapılandırıyoruz
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: rocket_cors::AllowedHeaders::all(),
        allow_credentials: false,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS yapılandırması başarısız oldu!");

    let rt = tokio::runtime::Runtime::new()
        .expect("failed to spawn tokio tuntime");

    let _handle = rt.handle().clone();

    rt.block_on(async move {
        let renderer = Renderer::new(opt.template_directory);
        let database = AppDatabase::new(&opt.connection_string).await;

        let config = clipstash::RocketConfig {
            renderer,
            database
        };

        clipstash::rocket(config)
        .attach(cors)
            .launch()
            .await
            .expect("failed to launch server")
    });


}