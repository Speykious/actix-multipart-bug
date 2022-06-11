use std::io::{Error, ErrorKind};

use actix_multipart::Multipart;
use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use futures_util::stream::TryStreamExt;
use tokio::io::AsyncBufReadExt;
use tokio_util::io::StreamReader;

#[post("/print-file")]
async fn print_file(mut payload: Multipart) -> Result<impl Responder, actix_web::Error> {
    while let Some(field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        println!("Name: {:?}", content_disposition.get_name());
        println!("File: {:?}", content_disposition.get_filename());
        println!("Type: {:?}", field.content_type());
        println!("===========================");

        let file_content_reader = StreamReader::new(
            field
                .into_stream()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e)),
        );

        let mut line_reader = file_content_reader.lines();
        let mut i = 0;
        while let Some(line) = line_reader.next_line().await? {
            println!("{:02x}: {}", i, line);
            i += 1;
        }

        println!("===========================\n");
    }
    println!();

    Ok(HttpResponse::Ok())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "127.0.0.1";
    let port = 7727;
    
    println!("Starting server on http://{}:{}/", url, port);
    HttpServer::new(|| App::new().service(print_file))
        .bind((url, port))?
        .run()
        .await
}
