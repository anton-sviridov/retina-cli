use diesel::prelude::*;
use crate::models::Detection;


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
        // println!("{}", detection.structure);
    }
}
