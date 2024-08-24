use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use reqwest;


// [Retina]struct for login
#[derive(Deserialize, Debug)]
struct Login {
    token: String,
    confirmed: bool,
}

// [Retina]struct for reports
#[derive(Serialize, Deserialize, Debug)]
struct FundusData {
    exudates_in_macula: bool,
    exudates_in_fovea: bool,
    height: i32,
    width: i32,
    image: String,
    macula: Vec<f32>,
    hard_exudates: Vec<Vec<i32>>,
    intraretinal_hemorrhages: Vec<Vec<i32>>,
    soft_exudates: Vec<Vec<i32>>,
    fibrose: Vec<Vec<i32>>,
    laser: Vec<Vec<i32>>,
    microaneurysms: Vec<Vec<i32>>,
    neovascularization: Vec<Vec<i32>>,
    preretinal: Vec<Vec<i32>>,
    va: Vec<Vec<i32>>,
}


// functions from retina
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

pub fn send_fundus(token: &str, image: &str) -> Result<FundusData, String> {
    let client = reqwest::blocking::Client::new();

    let mut fundus_data = HashMap::new();
    fundus_data.insert("token", token);
    fundus_data.insert("image", image);

    println!("{:#?}", fundus_data);

    let fundus_str_result = serde_json::to_string(&fundus_data);
    let fundus_str = match fundus_str_result {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };
    println!("{:#?}", fundus_str);

    let fundus_response_result = client
        .post("https://functions.yandexcloud.net/d4e5t0njkd4f1mb9kh5l")
        .body(fundus_str)
        .send();

    let fundus_response = match fundus_response_result {
        Ok(result) => result,
        Err(err) => return Err(err.to_string()),
    };
    println!("{:#?}", fundus_response);

    let fundus_response_struct_result = fundus_response.json::<FundusData>();
    let fundus_response_struct = match fundus_response_struct_result {
        Ok(result) => result,
        Err(error) => return Err(error.to_string()),
    };

    println!("{:#?}", fundus_response_struct);
    Ok(fundus_response_struct)
}
