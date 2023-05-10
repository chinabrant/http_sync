use flutter_rust_bridge::SyncReturn;
use reqwest::header::HeaderMap;
use serde_json::Value;
use std::collections::HashMap;

pub struct LoadResult {
    pub html: String,
    pub redirect_url: String,
}

pub struct UrlOption {
    pub method: Option<String>,
    pub charset: Option<String>,
    pub headers: Option<String>,
    // pub headers: Option<HashMap<String, String>>,
    pub retry: Option<i32>,
    // pub body: Option<HashMap<String, String>>,
    pub string_body: Option<String>, // TODO 支持其它类型body
    pub keywords: String,
    pub url_no_query: String,
    // pub querys: HashMap<String, String>,
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn load(option: UrlOption) -> SyncReturn<LoadResult> {
    if option.method.is_some() && option.method.as_ref().unwrap().to_string() == "POST".to_string()
    {
        post(option)
    } else {
        get(option)
    }
}

pub fn get(option: UrlOption) -> SyncReturn<LoadResult> {
    let client = reqwest::blocking::Client::new();

    // 组装header
    let mut headers = HeaderMap::new();
    if option.headers.is_some() {
        let header_string = option.headers.unwrap();
        if !header_string.is_empty() {
            let value: Value = serde_json::from_str(&header_string).unwrap();

            let header_map = value.as_object().unwrap();
            // let mut headers_map = HashMap::<String, String>::new();
            for (key, v) in header_map.iter() {
                // headers_map.insert(key.to_string(), v.to_string());
                headers.insert(
                    string_to_static_str(key.to_string()),
                    v.to_string().parse().unwrap(),
                );
            }
        }
    }

    // if option.headers.is_some() {
    // for (key, value) in option.headers.unwrap().iter() {
    //     headers.insert(
    //         string_to_static_str(key.to_string()),
    //         value.parse().unwrap(),
    //     );
    // }
    // }

    // 组装要提交的数据
    // let mut data = HashMap::new();
    // data = option.querys;

    let resp = client
        .get(option.url_no_query)
        .headers(headers)
        // .json(&data)
        .send();

    let mut result = LoadResult {
        html: "".to_string(),
        redirect_url: "".to_string(),
    };

    if resp.is_ok() {
        let text_resp = resp.unwrap().text();
        if text_resp.is_ok() {
            // let text = text_resp.unwrap();
            // println!("GET: {}", text);
            println!("GET: 请求成功");
            result = LoadResult {
                html: text_resp.unwrap(),
                redirect_url: "".to_string(),
            };
        }
    } else {
        println!("请求失败");
    }

    SyncReturn(result)
}

pub fn post(option: UrlOption) -> SyncReturn<LoadResult> {
    let client = reqwest::blocking::Client::new();

    // 组装header
    // let mut headers = HeaderMap::new();
    // if option.headers.is_some() {
    // for (key, value) in option.headers.unwrap().iter() {
    //     headers.insert(
    //         string_to_static_str(key.to_string()),
    //         value.parse().unwrap(),
    //     );
    // }
    // }

    // 组装要提交的数据
    // let mut data = HashMap::new();
    // data = option.querys;

    let resp = client
        .post(option.url_no_query)
        // .headers(headers)
        // .json(&data)
        .send();

    let mut result = LoadResult {
        html: "".to_string(),
        redirect_url: "".to_string(),
    };

    if resp.is_ok() {
        let text_resp = resp.unwrap().text();
        if text_resp.is_ok() {
            // let text = text_resp.unwrap();
            // println!("POST: {}", text);
            println!("POST: 请求成功");
            result = LoadResult {
                html: text_resp.unwrap(),
                redirect_url: "".to_string(),
            };
        }
    } else {
        println!("请求失败");
    }

    SyncReturn(result)
}
