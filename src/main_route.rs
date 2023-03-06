mod protonations;
use protonations::solvers::{self, MacroStateDistro};

use rocket::serde::{Deserialize, json::Json};

use self::protonations::types::IonisableGroup;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PostPayload {
    pub sequence: Vec<String>,
    pub ph_range: [f32; 3],
    pub tol: f32
}

#[post("/protonations", format="application/json", data="<payload>")]
pub fn index(payload: Json<PostPayload>) -> Result<Json<MacroStateDistro>, String> {

    let (sequence, failures): (Vec<_>, Vec<_>) = payload.sequence
        .iter()
        .map(|x| IonisableGroup::from_str(x).or_else(|_| Err(x)))
        .partition(Result::is_ok);

    let failures = failures.into_iter().map(Result::unwrap_err).collect::<Vec<_>>();
    let sequence = sequence.into_iter().map(Result::unwrap).collect::<Vec<_>>();

    if failures.len() > 0 {

        let err_str = format!(
            "The following lines failed to parse: {:?}",
            failures
        );

        return Err(err_str);
    }

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