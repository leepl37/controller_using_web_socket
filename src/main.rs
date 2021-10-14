use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::{Form};
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCheck {
    pub name:String,
    pub pass:String
}
use dotenv::dotenv;
use actix_web::middleware::Logger;
use sha256::digest;

#[get("/")]
async fn m_main(tera : web::Data<Tera> ) -> impl Responder {
    let mut context = Context::new();
    context.insert("info", "");
    // let rendered = tera.render("m_main.html", &context).unwrap();
    // let rendered = tera.render("index.html", &context).unwrap();

    //Web socket.
    //WEB_HEAD 와 SOCKET_URL를 받아와 통신한다.
    let web_socket_head = std::env::var("WEB_HEAD").expect("there are no web_head");
    let web_socket_addr = std::env::var("SOCKET_URL").expect("there are no web_socket_url");
    context.insert("head", &web_socket_head);
    context.insert("addr", &web_socket_addr);
    let rendered = tera.render("controller.html", &context).unwrap();
    // HttpResponse::Ok().header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN,":*").body(rendered)
    // use actix_http::{http, Request, Response};
    // HttpResponse::Ok().header("Access-Control-Allow-Origin", "*").body(rendered)
    HttpResponse::Ok().body(rendered)
    // HttpResponse::Ok().header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN,":*").body(rendered)
}

#[test]
fn test() {
    let string = digest("gab215215");
    println!("{}", string);
    //bcbd755910b19d4b8f5933cd9776e6b32b39be46b870f0ee8e9ea1e9b7f7a303
}

//DB를 활용하지 않기 때문에 .env 파일에 비밀번호를 지정 후 사용한다.
//현재 테스트용이기 때문에 주석 처리 후 아무 처리 없이 로그인 허용해둔 상태.
#[post("/")]
async fn m_main_post(data : Form<LoginCheck>,tera :web::Data<Tera>) -> impl Responder {
    dotenv().ok();
    // let ps_env = std::env::var("PW").expect("there are no password in env");
    // let id_env = std::env::var("ID").expect("there are no ID in env");
    // let input = &data.pass;
    // let val = digest(input);

    let mut context = Context::new();
    // if data.name == id_env && val ==  ps_env{
        let web_socket_head = std::env::var("WEB_HEAD").expect("there are no web_head");
        let web_socket_addr = std::env::var("SOCKET_URL").expect("there are no web_socket_url");
        context.insert("head", &web_socket_head);
        context.insert("addr", &web_socket_addr);
        let rendered = tera.render("controller.html", &context).unwrap();
        HttpResponse::Ok().body(rendered)
    // }else{
    //     context.insert("info", "정보가 일치하지 않습니다.");
    //     let rendered = tera.render("m_main.html", &context).unwrap();
    //     HttpResponse::Ok().body(rendered)
    // }
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(move || {
        let tera = Tera::new("templates/**/*").unwrap();
        // let cors = Cors::default().supports_credentials();

        App::new()
            // .wrap(cors)
            .wrap(Logger::default())
            .data(tera)
            .service(actix_files::Files::new("/assets", ".").show_files_listing())
            .service(m_main)
            .service(m_main_post)
    })
        .bind("0.0.0.0:8383")?
        .run()
        .await
}
