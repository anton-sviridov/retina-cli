use std::collections::HashMap;
use crate::retina::Login;

pub fn login(email: &str, password: &str) -> Result<String, String> {
    // println!("login, {}, {}", email, password);

    let client = reqwest::blocking::Client::new();

    let mut map = HashMap::new();
    map.insert("op", "login");
    map.insert("email", email);
    map.insert("password", password);

    // let json_string: String = serde_json::to_string(&map).unwrap();

    let json_result: Result<String, serde_json::Error> = serde_json::to_string(&map);
    let json_string: String = match json_result {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    // println!("{}", json_string);

    let resp_result = client
        .post("https://functions.yandexcloud.net/d4esr6ie5khkuno72hib")
        // .json(&map)
        .body(json_string)
        .send();

    let resp = match resp_result {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    // println!("{resp:#?}");

    // let resp_string: String = resp.text().unwrap();
    // println!("{resp_string:#?}");

    let resp_struct_result: Result<Login, reqwest::Error> = resp.json::<Login>();

    let resp_struct = match resp_struct_result {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    // println!("{resp_struct:#?}");

    // format!("Hello, {}! You've been greeted from Rust!", email)
    Ok(resp_struct.token)
}
