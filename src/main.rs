use std::collections::HashMap;
use lambda_http::{run, service_fn, Body, Error, Request, http, Response};
use crate::http::HeaderValue;
use serde::{Deserialize, Serialize};

// backend jest zdeployowany na AWS Lambda, a front na vercelu wiec nie musisz odpalac u siebie

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cipher_key = "gjAiTr|u?m&$FJn.>z'WyP{:xYhB;qZ!w)eM[-V%NaCo_sX8}L*Q";

    let cipher_map = cipher_map(cipher_key);
    let decipher_map = decipher_map(&cipher_map);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(move |event| { 
        lambda_path_handler(event, cipher_map.clone(), decipher_map.clone()) 

    })).await
}



async fn lambda_path_handler(event: Request, cipher_map: HashMap<char, char>, decipher_map: HashMap<char, char>)  -> Result<Response<Body>, Error> {
    let controller = CipherController;

    match (event.uri().path(), event.method().as_str()) {
        // path | method => handler_function(event, map).await
        ("/cipher", "POST") => CipherController::cipher(&controller, event, cipher_map).await,
        ("/decipher", "POST") => CipherController::decipher(&controller, event, decipher_map).await,

        _ => Ok(build_response(404, LambdaResponse{ text: String::from("Not Found") }))

    }
}



fn build_response(status_code: u16, body: LambdaResponse) -> Response<Body> {
    let mut response = Response::builder()
        .status(status_code)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    response.headers_mut().insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    response.headers_mut().insert("Access-Control-Allow-Methods", HeaderValue::from_static("POST"));
    response.headers_mut().insert("Access-Control-Allow-Headers", HeaderValue::from_static("*"));
    response.headers_mut().insert("Access-Control-Max-Age", HeaderValue::from_static("86400")); 
    response.headers_mut().insert("Content-Type", HeaderValue::from_static("application/json"));
    
    response 
}



struct CipherController;

impl CipherController {
    async fn cipher(&self, event: http::Request<Body>, cipher_map: HashMap<char, char>) ->  Result<Response<Body>, Error> {
        let request_body = event.into_body();
    
        let request_json: LambdaRequest = serde_json::from_slice(&request_body.to_vec()).unwrap();
        let request_text = request_json.text;

        let cipher: String = request_text.chars().map(|text_char| {
            if text_char.is_alphabetic() {
                *cipher_map.get(&text_char).unwrap_or(&text_char)
            } else { text_char }

        }).collect();
        
        Ok(build_response(200, LambdaResponse{
            text: cipher
        }))
    }


    async fn decipher(&self, event: http::Request<Body>, decipher_map: HashMap<char, char>) ->  Result<Response<Body>, Error> {
        let request_body = event.into_body();
    
        let request_json: LambdaRequest = serde_json::from_slice(&request_body.to_vec()).unwrap();
        let request_text = request_json.text;

        let decipher: String = request_text.chars().map(|text_char| {
            *decipher_map.get(&text_char).unwrap_or(&text_char)
            
        }).collect();

        Ok(build_response(200, LambdaResponse{
            text: decipher
        }))
    }

}



#[derive(Deserialize, Serialize)]
struct LambdaResponse{
    text: String
}

#[derive(Deserialize, Serialize)]
struct LambdaRequest{
    text: String
}



fn cipher_map(key: &str) -> HashMap<char, char> {
    let alphabeth = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut cipher_map = HashMap::new();

    for (char, key_char) in alphabeth.chars().zip(key.chars()) { cipher_map.insert(char, key_char); }

    cipher_map
}



fn decipher_map(map: &HashMap<char, char>) -> HashMap<char, char> {
    let mut decipher_map = HashMap::new();

    for (&key, &wartosc) in map { decipher_map.insert(wartosc, key); }

    decipher_map
}