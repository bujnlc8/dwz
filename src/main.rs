use serde::{Deserialize, Serialize};

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use dwz::controllers::{get_redirect_url, insert_data};

use tera::{Context, Tera};

#[derive(Serialize, Deserialize)]
pub struct Params {
    long_url: String,
    valid_time: String,
}

async fn index(path: web::Path<(String,)>) -> HttpResponse {
    let query = &path.0;
    let redirect_url = get_redirect_url(query);
    if redirect_url.len() <= 0 {
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
    let mut ctx = Context::new();
    ctx.insert("name", "hello");
    let rendered = tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

async fn shorten(params: web::Form<Params>) -> Result<HttpResponse> {
    let short = insert_data(&params.long_url, &params.valid_time);
    match short {
        Ok(e) => Ok(HttpResponse::Ok()
            .content_type("text/html;charset=utf8")
            .body(format!(
                "原始链接:{}<br>短链接:dwz0.tk/{}<br><a href='/'>返回</a>",
                params.long_url, e
            ))),
        Err(e) => Ok(HttpResponse::Ok()
            .content_type("text/html;charset=utf8")
            .body(format!(
                "Something went wrong, {} <br><a href='/'>返回</a>",
                e,
            ))),
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
            .service(web::resource("/").route(web::get().to(render_admin)))
            .service(web::resource("").route(web::get().to(render_admin)))
            .route("/{path}", web::get().to(index))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
