use serde::{Deserialize, Serialize};

pub mod login;
pub mod send_fundus;


// [Retina]struct for login
#[derive(Deserialize, Debug)]
pub struct Login {
    token: String,
    confirmed: bool,
}

// [Retina]struct for reports
#[derive(Serialize, Deserialize, Debug)]
pub struct FundusData {
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