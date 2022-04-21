use serde::{Deserialize, Serialize};

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use dwz::controllers::{get_redirect_url, insert_data};

use tera::{Context, Tera};

#[derive(Serialize, Deserialize)]
pub struct Params {
    long_url: String,
    valid_time: Option<String>,
}

async fn index(path: web::Path<(String,)>) -> HttpResponse {
    let query = &path.0;
    let redirect_url = get_redirect_url(query);
    if redirect_url.is_empty() {
        return HttpResponse::NotFound().finish();
    }
    HttpResponse::Found()
        .insert_header(("location", redirect_url))
        .finish()
}

fn get_bind_port() -> u16 {
    match std::env::var("DWZ_PORT") {
        Err(_e) => 8080u16,
        Ok(s) => s.parse::<u16>().unwrap(),
    }
}

async fn render_admin(tmpl: web::Data<Tera>, _req: HttpRequest) -> HttpResponse {
    let ctx = Context::new();
    let rendered = tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

fn get_valid_time(params: &web::Form<Params>) -> String {
    let mut valid_time = "2222-02-22 22:22:22".to_string();
    if let Some(e) = &params.valid_time {
        let s = e.to_owned().trim().to_string();
        if !s.is_empty() {
            valid_time = s;
        }
    }
    valid_time
}

fn get_full_url(path: &str) -> String {
    match std::env::var("DWZ_HOST") {
        Err(_) => format!("dwz0.tk/{}", path),
        Ok(e) => format!("{}/{}", e, path),
    }
}

async fn shorten(params: web::Form<Params>) -> Result<HttpResponse> {
    match insert_data(&params.long_url, &get_valid_time(&params)) {
        Ok(e) => Ok(HttpResponse::Ok()
            .content_type("text/html;charset=utf8")
            .body(format!(
                "原始链接: <span style='word-wrap: break-word;color:grey;'>{}</span><br>短链接: {}<br><a href='/'>返回</a>",
                params.long_url.trim(),
                get_full_url(e.as_str()),
            ))),
        Err(e) => Ok(HttpResponse::Ok()
            .content_type("text/html;charset=utf8")
            .body(format!(
                "Oooops...Something went wrong, {} <br><a href='/'>返回</a>",
                e,
            ))),
    }
}

#[derive(Serialize)]
pub struct Resp {
    success: bool,
    data: Option<String>,
    msg: Option<String>,
}

async fn api_shorten(params: web::Form<Params>) -> Result<HttpResponse> {
    match insert_data(&params.long_url, &get_valid_time(&params)) {
        Ok(e) => Ok(HttpResponse::Ok().json(Resp {
            success: true,
            data: Some(get_full_url(e.as_str())),
            msg: None,
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(Resp {
            success: false,
            data: None,
            msg: Some(format!("{}", e)),
        })),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let port = get_bind_port();
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .app_data(web::Data::new(tera))
            .wrap(middleware::Logger::default())
            .service(web::resource("/shorten").route(web::post().to(shorten)))
            .service(web::resource("/url").route(web::post().to(api_shorten)))
            .service(web::resource("/").route(web::get().to(render_admin)))
            .service(web::resource("").route(web::get().to(render_admin)))
            .route("/{path}", web::get().to(index))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
