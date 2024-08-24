// use clap::{Parser, Subcommand};
use diesel::prelude::*;
// use std::path::PathBuf;
use crate::models::{Detection, NewDetection};
use crate::retina::{login, send_fundus};




// cli functions
pub fn report(database: &str) {
    // TODO: check if database exists. If does, go on. If does not, print "database does not exists";

    println!("{}", database);
    use crate::schema::detections::dsl::*;

    let connection = &mut SqliteConnection::establish(database).unwrap();
    // Run "SELECT * FROM detections";
    let results = detections
        .limit(5)
        .select(Detection::as_select())
        .load(connection)
        .expect("Error loading detections");

    // Print query result in CLI
    println!("Displaying {} detections", results.len());
    for detection in results {
        println!("{}", detection.image);
        println!("-----------\n");
        println!("{}", detection.description);
    }
}

// TODO: add flags email, password, image
pub fn detect(file: &str, comment: &str, database: &str) {
    // TODO: Call a function login with parameters email (&str) and password (&str). If successfull, the function returns a token;
    // let token: &str = login(email, password);

    // TODO: Convert an image to webp;
    // TODO: Call a function send_fundus with token (&str) and webp image in data url (&str). If successfull, get result (struct FundusData), if not — print an error message;
    // TODO: Check if database with table exists. If not, initialize it first.
    // TODO: Rename an image to hash name

    // insert fundus data and hash name of image to table;
    use crate::schema::detections;
    let connection = &mut SqliteConnection::establish(database).unwrap();

    let new_memory = NewDetection {
        image: file,
        description: comment,
        date: "",
    };

    diesel::insert_into(detections::table)
        .values(&new_memory)
        .returning(Detection::as_returning())
        .get_result(connection)
        .expect("Error saving new post");

    // TODO: Check if folder cache exists. If not, create it first;
    // TODO: Copy an image with hash name in folder cache.
}