mod protonations;
use protonations::sequence_parser::Config;
use protonations::solvers::{self, MacroStateDistro};

use rocket::serde::{Deserialize, json::Json};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PostPayload<'r> {
    pub sequence: &'r str,
    pub ph_range: [f32; 3],
    pub tol: f32
}

#[post("/protonations", format="application/json", data="<payload>")]
pub fn index(payload: Json<PostPayload<'_>>) -> Result<Json<MacroStateDistro>, String> {

    let config = Config::build("config")?;

    let sequence = config.parse_sequence(&payload.sequence);

    if payload.ph_range[0] >= payload.ph_range[1] {
        
        let err_str = format!(
            "Invalid boundaries for pH range! {} should be smaller than {}",
            payload.ph_range[0],
            payload.ph_range[1]
        );

        return Err(err_str);
    }

    if payload.ph_range[1] - payload.ph_range[0] < payload.ph_range[2] {

        let err_str = format!(
            "Invalid step size for pH range! ({}, {}) should contain {}",
            payload.ph_range[0],
            payload.ph_range[1],
            payload.ph_range[2]
        );

        return Err(err_str);
    }

    let mut ph_values = vec![payload.ph_range[0], ];
    while *ph_values.last().unwrap() < payload.ph_range[1] {
        ph_values.push(ph_values.last().unwrap() + payload.ph_range[2]);
    }

    let solution = solvers::solve_range(&sequence, &ph_values, payload.tol);

    Ok(Json(solution))
}