use super::types::*;

use std::collections::HashMap;

/// Find all of the relevant microstates for a vector of ionisable groups.
/// It is called solve_point, because it solves the microstate distribution problem for a single pH point.
/// 
/// - sequence: The vector of ionisable groups.
/// - h_ion_conc: The H+ ion concentration.
/// - tol: The % of tolerance below we discard the microstates.
/// 
/// Returns a vector of relevant microstates (prevalence > tol).
pub fn solve_point(sequence: &[IonisableGroup], h_ion_conc: f32, tol: f32) -> Vec<MicroState> {

    let mut output = vec![MicroState { prevalence: 1.0, charge_pattern: Vec::new() }];

    for group in sequence {

        let group_charges = group.gco.to_tuple();
        let coeff = h_ion_conc / (h_ion_conc + group.ka);

        // Create high pH microstates
        let mut high_ph_states = Vec::new();
        for state in &output {

            let mut new_state = state.clone();
            new_state.charge_pattern.push(group_charges.1);
            new_state.prevalence *= 1. - coeff;

            if new_state.prevalence > tol { high_ph_states.push(new_state); }  // Filter out low population high pH states            
        }

        // Modify existing microstates to low pH microstates
        let mut low_population_states = Vec::new();
        for (idx, state) in output.iter_mut().enumerate() {

            state.charge_pattern.push(group_charges.0);
            state.prevalence *= coeff;

            if state.prevalence < tol { low_population_states.push(idx); }
        }

        // Filter out low population low pH states  
        for &idx in low_population_states.iter().rev() {
            output.remove(idx);
        }

        // Add the high pH microstates to the low pH microstates
        output.append(&mut high_ph_states);
    }

    output
}

pub type MacroStateDistro = Vec<HashMap<i32, f32>>;

/// Find all of the relevant macrostates for a vector of ionisable groups.
/// It is called solve_range, because it solves the macrostates distribution problem for multiple pH points.
/// 
/// - sequence: The vector of ionisable groups.
/// - ph_values: The pH values we want to calcule the macrostates distributions for.
/// - tol: The % of tolerance below we discard the microstates.
/// 
/// Returns a vector of macrostate distributions in the form of hash maps, for which the keys are the
/// macrostate charges, while the values are the total macrostate prevalences.
pub fn solve_range(sequence: &[IonisableGroup], ph_values: &[f32], tol: f32) -> MacroStateDistro {

    let mut output = Vec::new();

    for ph in ph_values {

        let mut charge_prevalences = HashMap::new();

        let h_ion_conc = 10f32.powf(-ph);
        let microstates = solve_point(sequence, h_ion_conc, tol);

        for state in microstates {

            let total_charge = state.charge_pattern.iter().sum::<i32>();
            charge_prevalences.entry(total_charge).and_modify(|prevalence| {
                *prevalence += state.prevalence;
            }).or_insert(state.prevalence);
        }

        output.push(charge_prevalences);
    }

    output

}