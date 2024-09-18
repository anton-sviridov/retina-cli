use std::fmt::format;

use crate::models::{Detection, NewDetection};
use diesel::prelude::*;

use crate::retina::convert::convert_dataurl;
use crate::retina::{login::login, send_fundus::send_fundus};

use crate::retina::FundusData;
use image::DynamicImage;


pub fn detect(database: &str, email: &str, password: &str, path: &str) {
    let token = login(email, password).unwrap();
    // println!("token: {token}");

    // get file from image path
    let image: DynamicImage = image::open(path).unwrap();
    



    // convert an image to base64 Data URL in webp — call retina/convert.rs
    // let dataurl = format!("data:image/webp;base64,{}", convert_dataurl(image));
    
    let dataurl = convert_dataurl(image);
    println!("{dataurl}");
    
    
    // TODO: Call a function send_fundus with token (&str) and dataurl (&str). If successfull, get result (struct FundusData), if not — print an error message;
    let fundus_response = send_fundus(token.as_str(), dataurl.as_str()).unwrap();

    println!("{:?}", fundus_response);
    // TODO: Check if database with table exists. If not, initialize it first.
    
    
    
    // TODO: Rename an image to hash name

    // insert fundus data and hash name of image to table;

    /*
        use crate::schema::detections;
        let connection = &mut SqliteConnection::establish(database).unwrap();

        let new_memory = NewDetection {
            image,
            date: "",
            structure: String::from("FundusData"),
        };

        diesel::insert_into(detections::table)
            .values(&new_memory)
            .returning(Detection::as_returning())
            .get_result(connection)
            .expect("Error saving new post");
    */

    // TODO: Check if folder cache exists. If not, create it first;
    // TODO: Copy an image with hash name in folder cache.
}
