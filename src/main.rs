mod openai;
// mod http2;
use std::{
    net::TcpStream, 
    io::{
        Read, 
        Write
    }, str::FromStr
};
use serde::{Serialize, Deserialize, de::Error};
use tokio_stream::StreamExt;
use tokio::io::AsyncRead;



struct HTTPRequest<Body: Serialize> {
    method: String,
    path: String,
    host: String,
    headers: Vec<(String, String)>,
    body: Body,
}

impl<Body: Serialize> HTTPRequest<Body> {
    fn new(method: String, path: String, host: String, body: Body) -> Self {
        Self {
            method,
            path,
            host,
            headers: Vec::new(),
            body,
        }
    }
    fn add_header(mut self, key: String, value: String) -> Self{
        self.headers.push((key, value));
        self
    }
    fn to_string(&self) -> String {
        let mut request = format!("{} {} HTTP/1.1\r", self.method, self.path);
        request.push_str(&format!("Host: {}\r", self.host));
        for (key, value) in &self.headers {
            request.push_str(&format!("{}: {}\r", key, value));
        }
        let body = serde_json::to_string(&self.body).unwrap();
        request.push_str(&format!("Content-Length: {}\r", body.len()));
        request.push_str("Content-Type: application/json\r");
        request.push_str("Accept: application/json\r");
        request.push_str("User-Agent: curl/7.87.0\r");
        request.push_str("\r");
        request.push_str(&serde_json::to_string(&self.body).unwrap());
        request
    }
}

fn main() {
    let stream = std::fs::read_to_string("example.txt").expect("Failed to read file");
    let result = stream
        .split("\n\n")
        .filter(|item| !item.is_empty())
        .map(openai::ChatResponse::from_str)
        .filter_map(|item| {
            match item {
                Ok(item) => Some(item),
                Err(e) => {
                    if cfg!(debug_assertions) {
                        println!("Error: {:?}", e); 
                    }
                    None
                },
            }
        })
        .for_each(|item| print!("{}", item));


}
// #[tokio::main]
async fn main2() -> std::io::Result<()> {
    let openai_api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    // Send a request
    let chat_request = openai::ChatRequest::new("gpt-3.5-turbo".to_string())
        .add_message(openai::Role::System, "You are ChatGPT, a powerful conversational chatbot. Answer to my questions in informative way.".to_string())
        .add_message(openai::Role::User, "Hi, how are you ?".to_string())
        .into_stream();

    let client = reqwest::Client::new();
    let mut stream = client.post("https://api.openai.com/v1/chat/completions")
        .header("User-Agent", "openai-rs/1.0")
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .json(&chat_request)
        .send()
        .await
        .expect("Failed to send request")
        .bytes_stream();
    
    // while let Some(item) = stream.next()
    //     .map(|item| item.ok())
    //     .flatten()
    //     .map(|item| String::from_utf8_lossy(item.as_ref())
    //         .split("\n\n")
    //         .map(|item| String::from(item))
    //         .map(|item| serde_json::from_str::<openai::ChatResponse>(&item))
    //         .collect::<Vec<_>>()) {
    //         println!("Chunk: {:?}", item)
    // }
    Ok(())
}