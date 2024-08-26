use diesel::prelude::*;
use crate::models::{Detection, NewDetection};
use crate::retina::login::login;


pub fn detect(database: &str, email: &str, password: &str, path: &str) {
    let token = login(email, password).unwrap();
    println!("token: {token}");
    
    // get file from image path
    // let image: DynamicImage = image::open(path).unwrap();

    // TODO: convert an image to base64 Data URL in webp — call retina/convert.rs
    // let dataurl = convert_dataurl(image);

    
    // TODO: Call a function send_fundus with token (&str) and dataurl (&str). If successfull, get result (struct FundusData), if not — print an error message;
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