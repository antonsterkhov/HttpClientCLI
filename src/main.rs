use clap::{Parser, Subcommand};
use reqwest::blocking::{Client, multipart};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use anyhow::Result;

/// HTTP CLI: Отправка запросов GET, POST, PUT, DELETE с возможностью добавления заголовков и отправки файлов.
///
/// # Примеры использования:
///
/// ## GET-запрос:
/// ```sh
/// http_client get http://example.com
/// http_client get example.com -H "User-Agent=MyClient"
/// ```
///
/// ## POST-запрос:
/// ```sh
/// http_client post http://example.com -d '{"key": "value"}' -H "Content-Type=application/json"
/// http_client post example.com -f file.txt
/// ```
///
/// ## PUT-запрос:
/// ```sh
/// http_client put http://example.com -d '{"update": true}'
/// http_client put example.com -f update.txt
/// ```
///
/// ## DELETE-запрос:
/// ```sh
/// http_client delete http://example.com
/// ```
#[derive(Parser)]
#[command(name = "HttpClientCLI", version = "1.1", about = "CLI для работы с HTTP")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// GET-запрос
    #[command(about = "Отправка GET-запроса на указанный URL.",
        long_about = "Отправляет GET-запрос на указанный URL с возможностью указания заголовков.\n\nПример:\n\nhttp_client get http://example.com\nhttp_client get example.com -H \"User-Agent=MyClient\"")]
    Get {
        #[arg(value_name = "URL")]
        url: String,

        #[arg(short = 'H', long, value_parser = parse_key_val)]
        headers: Vec<(String, String)>,
    },

    /// POST-запрос
    #[command(about = "Отправка POST-запроса с данными или файлом.",
        long_about = "Отправляет POST-запрос на указанный URL. Можно отправить JSON-данные или загрузить файл.\n\nПример с данными:\n\nhttp_client post http://example.com -d '{\"key\": \"value\"}' -H \"Content-Type=application/json\"\n\nПример с файлом:\n\nhttp_client post example.com -f file.txt")]
    Post {
        #[arg(value_name = "URL")]
        url: String,

        #[arg(short, long)]
        data: Option<String>,

        #[arg(short, long)]
        file: Option<PathBuf>,

        #[arg(short = 'H', long, value_parser = parse_key_val)]
        headers: Vec<(String, String)>,
    },

    /// PUT-запрос
    #[command(about = "Отправка PUT-запроса с данными или файлом.",
        long_about = "Отправляет PUT-запрос на указанный URL. Можно отправить обновленные данные в формате JSON или загрузить файл.\n\nПример с данными:\n\nhttp_client put http://example.com -d '{\"update\": true}'\n\nПример с файлом:\n\nhttp_client put example.com -f update.txt")]
    Put {
        #[arg(value_name = "URL")]
        url: String,

        #[arg(short, long)]
        data: Option<String>,

        #[arg(short, long)]
        file: Option<PathBuf>,

        #[arg(short = 'H', long, value_parser = parse_key_val)]
        headers: Vec<(String, String)>,
    },

    /// DELETE-запрос
    #[command(about = "Отправка DELETE-запроса на указанный URL.",
        long_about = "Отправляет DELETE-запрос на указанный URL с возможностью указания заголовков.\n\nПример:\n\nhttp_client delete http://example.com")]
    Delete {
        #[arg(value_name = "URL")]
        url: String,

        #[arg(short = 'H', long, value_parser = parse_key_val)]
        headers: Vec<(String, String)>,
    },
}

fn parse_key_val(s: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        anyhow::bail!("Формат заголовка: ключ=значение");
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

fn build_headers(headers: &[(String, String)]) -> HeaderMap {
    let mut header_map = HeaderMap::new();
    for (key, value) in headers {
        if let (Ok(name), Ok(val)) = (HeaderName::from_bytes(key.as_bytes()), HeaderValue::from_str(value)) {
            header_map.insert(name, val);
        }
    }
    header_map
}

fn ensure_url_prefix(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        format!("http://{}", url)
    }
}

fn main() {
    let args = Cli::parse();
    let client = Client::builder().timeout(Duration::from_secs(10)).build().expect("Не удалось создать клиент");

    match args.command {
        Commands::Get { url, headers } => {
            let url = ensure_url_prefix(&url);
            let response = client.get(&url).headers(build_headers(&headers)).send();
            handle_response(response);
        }
        Commands::Post { url, data, file, headers } => {
            let url = ensure_url_prefix(&url);
            let response = if let Some(file_path) = file {
                let file_content = fs::read(file_path).expect("Не удалось прочитать файл");
                let form = multipart::Form::new().part("file", multipart::Part::bytes(file_content));
                client.post(&url).headers(build_headers(&headers)).multipart(form).send()
            } else {
                let mut req = client.post(&url).headers(build_headers(&headers));
                if let Some(json_data) = data {
                    req = req.header(CONTENT_TYPE, "application/json").body(json_data);
                }
                req.send()
            };
            handle_response(response);
        }
        Commands::Put { url, data, file, headers } => {
            let url = ensure_url_prefix(&url);
            let response = if let Some(file_path) = file {
                let file_content = fs::read(file_path).expect("Не удалось прочитать файл");
                let form = multipart::Form::new().part("file", multipart::Part::bytes(file_content));
                client.put(&url).headers(build_headers(&headers)).multipart(form).send()
            } else {
                let mut req = client.put(&url).headers(build_headers(&headers));
                if let Some(json_data) = data {
                    req = req.header(CONTENT_TYPE, "application/json").body(json_data);
                }
                req.send()
            };
            handle_response(response);
        }
        Commands::Delete { url, headers } => {
            let url = ensure_url_prefix(&url);
            let response = client.delete(&url).headers(build_headers(&headers)).send();
            handle_response(response);
        }
    }
}

fn handle_response(response: Result<reqwest::blocking::Response, reqwest::Error>) {
    match response {
        Ok(resp) => {
            println!("Статус: {}", resp.status());
            println!("Заголовки ответа:");
            for (key, value) in resp.headers().iter() {
                println!("{}: {:?}", key, value);
            }
            match resp.text() {
                Ok(text) => println!("Ответ:\n{}", text),
                Err(e) => eprintln!("Ошибка чтения ответа: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Ошибка запроса: {}\nДетали: {:?}", e, e);
        }
    }
}
