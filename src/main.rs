use actix_web::{get, App, Error, HttpResponse, HttpServer, middleware, web};
use bgpkit_parser::{BgpElem, BgpkitParser};
use bgpkit_parser::parser::ElemType;
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
    let parser = match BgpkitParser::new(info.file.as_str()){
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
    let max = info.max.unwrap_or(100);
    let elems = parser.into_elem_iter()
        .filter(|elem| {
            if let Some(p) = &info.prefix {
                if elem.prefix.to_string() != *p{
                    return false
                }
            }
            if let Some(asn) = &info.asn {
                if let Some(origins)  = &elem.origin_asns {
                    if !origins.contains(asn) {
                        return false
                    }
                } else {
                    return false
                }
            }
            if let Some(msg_type) = &info.msg_type {
                match msg_type.to_lowercase().as_str() {
                    "announcement" | "announce" | "a" => {
                        if let ElemType::WITHDRAW = elem.elem_type {
                            return false
                        }
                    }
                    "withdrawal" | "withdraw" | "w" => {
                        if let ElemType::ANNOUNCE = elem.elem_type {
                            return false
                        }
                    }
                    _ => return true
                }
            }
            true
        })
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

