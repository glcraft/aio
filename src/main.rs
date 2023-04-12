mod openai;
// mod http2;
use std::{
    net::TcpStream, 
    io::{
        Read, 
        Write
    }
};
use serde::Serialize;



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

fn main() -> std::io::Result<()> {
    return Ok(());
    let openai_api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    // Send a request
    let chat_request = openai::ChatRequest::new("gpt-3.5-turbo".to_string())
        .add_message(openai::Role::System, "You are ChatGPT, a powerful conversational chatbot. Answer to my questions in informative way.".to_string())
        .add_message(openai::Role::User, "Hi, how are you ?".to_string());
    
    let request = HTTPRequest::new(
        "POST".to_string(),
        "/v1/chat/completions".to_string(),
        "api.openai.com".to_string(),
        chat_request
    ).add_header("Authorization".to_string(), format!("Bearer {}", openai_api_key));
    let request = request.to_string();
    println!("Request:\n{}\n\n", request.replace("\r", "\n"));
    //write the request to a file
    std::fs::write("request.txt", &request).expect("Failed to write request to file");
    // return Ok(());
    
    // Connect to OpenAI API
    let mut stream = TcpStream::connect("api.openai.com:80").expect("Failed to connect to OpenAI API");
    stream.write(request.as_bytes())?;
    let mut buf = [0; 512];
    stream.read(&mut buf)?;
    println!("Response:\n{}", String::from_utf8_lossy(&buf[..]));
    println!("Done!");
    Ok(())
}
