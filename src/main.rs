use actix_web::{get, App, Error, HttpResponse, HttpServer, middleware, web};
use bgpkit_parser::{BgpElem, BgpkitParser};
use serde::{Deserialize};
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct Info {
    file: String,
    msg_type: Option<String>,
    asn: Option<u32>,
    prefix: Option<String>,
    max: Option<usize>,
}

/// Search MRT data items.
#[get("/parse")]
async fn parse_item(
    info: web::Query<Info>
) -> Result<HttpResponse, Error> {
    let mut parser = match BgpkitParser::new(info.file.as_str()){
        Ok(p) => {p}
        Err(err) => {
            return Ok(
                HttpResponse::Ok().json(json!({
                "data": null,
                "error": err.to_string(),
            }))
            )
        }
    };


    if let Some(p) = &info.prefix {
        parser = parser.add_filter("prefix", p.as_str()).unwrap();
    }

    if let Some(asn) = &info.asn {
        parser = parser.add_filter("origin_asn", asn.to_string().as_str()).unwrap();
    }

    if let Some(msg_type) = &info.msg_type {
        match msg_type.to_lowercase().as_str() {
            "announcement" | "announce" | "a" => {
                parser=parser.add_filter("type", "a").unwrap();
            }
            "withdrawal" | "withdraw" | "w" => {
                parser=parser.add_filter("type", "w").unwrap();
            }
            _ => {}
        }
    }


    let max = info.max.unwrap_or(100);
    let elems = parser.into_elem_iter()
        .take(max).collect::<Vec<BgpElem>>();


    Ok(
        HttpResponse::Ok().json(json!({
                "data": elems,
                "error": null,
            }))
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let bind = "0.0.0.0:8080";

    println!("Starting server at: {}", &bind);
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(parse_item)
    })
        .bind(&bind)?
        .run()
        .await
}

