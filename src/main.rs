use clap::{Parser, Subcommand};
use std::path::PathBuf;
use retina_cli::models::{Detection, NewDetection};
use diesel::prelude::*;
use retina_cli::*;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

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


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    #[arg(short, long)]
    database: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // TODO: add flags email, password, image
    Detect {
        #[arg(short, long, required=true)]
        file: String,

        #[arg(short, long, required=true)]
        comment: String,

        #[arg(short, long, required=true)]
        database: String,
    },
    Report {
        #[arg(short, long, required=true)]
        database: String,
    }
}

// functions from retina
fn login(email: &str, password: &str) -> Result<String, String> {
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

fn send_fundus(token: &str, image: &str) -> Result<FundusData, String> {
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



fn report(database: &str) {
    // TODO: check if database exists. If does, go on. If does not, print "database does not exists";

    println!("{}", database);
    use self::schema::detections::dsl::*;

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
fn detect(file: &str, comment: &str, database: &str) {
    // TODO: Call a function login with parameters email (&str) and password (&str). If successfull, the function returns a token;
    // let token: &str = login(email, password);


    // TODO: Convert an image to webp;
    // TODO: Call a function send_fundus with token (&str) and webp image in data url (&str). If successfull, get result (struct FundusData), if not — print an error message;
    // TODO: Check if database with table exists. If not, initialize it first.
    // TODO: Rename an image to hash name
        
    
    // insert fundus data and hash name of image to table;
    use self::schema::detections;
    let connection = &mut SqliteConnection::establish(database).unwrap();


    let new_memory = NewDetection { image: file, description: comment, date: "" };

    diesel::insert_into(detections::table)
        .values(&new_memory)
        .returning(Detection::as_returning())
        .get_result(connection)
        .expect("Error saving new post");

    // TODO: Check if folder cache exists. If not, create it first;
    // TODO: Copy an image with hash name in folder cache.
}

fn main() {
    let cli = Cli::parse();

    // To check provided values
    if let Some(database) = cli.database.as_deref() {
        println!("Value for name: {}", database);
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }



    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Report { database }) => {
            report(database)
        },
        Some(Commands::Detect { file, comment, database }) => {
            detect(file, comment, database)
        },
        None => {},
    }
}