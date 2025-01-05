use clipstash::domain::maintance::Maintance;
use clipstash::web::renderer::Renderer;
use clipstash::{data::AppDatabase, web::hitcounter::HitCounter};
use dotenv::dotenv;
use std::path::PathBuf;
use structopt::StructOpt;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[derive(StructOpt, Debug)]
#[structopt(name = "httpd")]
struct Opt {
    #[structopt(default_value = "sqlite:data.db")]
    connection_string: String,
    #[structopt(short, long, parse(from_os_str), default_value = "templates/")]
    template_directory: PathBuf,
}

fn main() {
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

    let rt = tokio::runtime::Runtime::new().expect("failed to spawn tokio tuntime");

    let handle = rt.handle().clone();

    let renderer = Renderer::new(opt.template_directory.clone());
    let database = rt.block_on(async move { AppDatabase::new(&opt.connection_string).await });
    let hit_counter = HitCounter::new(database.get_pool().clone(), handle.clone());
    let maintance = Maintance::spawn(database.get_pool().clone(), handle.clone());

    let config = clipstash::RocketConfig {
        renderer,
        database,
        hit_counter,
        maintance
    };

    rt.block_on(async move {
        clipstash::rocket(config)
            .attach(cors)
            .launch()
            .await
            .expect("failed to launch server")
    });
}
