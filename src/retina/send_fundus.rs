use std::collections::HashMap;
use crate::retina::FundusData;


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
