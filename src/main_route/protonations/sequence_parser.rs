use super::types::*;
use std::{fs, collections::HashMap};

pub struct Config {
    pub groups: HashMap<String, IonisableGroup>
}

fn parse_group_values(
    group_values: Vec<&str>, 
    idx: usize, 
    line: &str
) -> Result<Vec<f32>, String> {

    let group_values = group_values
        .iter()
        .map(|&x| x.parse::<f32>())
        .collect::<Vec<_>>();

    if group_values.iter().any(|val| val.is_err()) {

        let err_str = format!(
            "[ Config::build ] Parsing to float failed at line index {} (line value: \"{}\")", 
            idx,
            line
        );

        return Err(err_str);
    }

    let group_values = group_values
        .into_iter()
        .map(|x| 10f32.powf(-x.unwrap()))
        .collect::<Vec<_>>();

    Ok(group_values)
}

fn insert_group(
    groups: &mut HashMap<String, IonisableGroup>,
    name_prefix: &str,
    name: &str,
    group_props: IonisableGroup,
    line_idx: usize
) -> Result<(), String> {

    if groups.insert(format!("{}{}", name_prefix, name), group_props).is_some() {

        let err_str = format!(
            "[ Config::build ] Key {}{} already exists at line index {}!", 
            name_prefix, 
            name, 
            line_idx
        );

        return Err(err_str);
    }
    Ok(())
}

fn get_gco(
    group_values: &mut Vec<&str>, 
    idx: usize
) -> Result<GroupChargeOptions, String> {

    match group_values.pop() {
        
        Some("NeuOrNeg") => Ok(GroupChargeOptions::NeuOrNeg),
        Some("PosOrNeu") => Ok(GroupChargeOptions::PosOrNeu),

        Some(x) => {

            let err_str = format!(
                "[ Config::build ] Invalig group charge option at line index {} (line value: \"{}\")", 
                idx, 
                x
            );

            Err(err_str)
        },

        None => unreachable!()
    }
}

impl Config {

    pub fn build(config_path: &str) -> Result<Self, String> {

        let config = fs::read_to_string(config_path);
        
        if let Err(err) = config {

            let err_str = format!(
                "[ Config::build ] Error during reading the config file! Message:\n{}", 
                err
            );

            return Err(err_str);
        }

        let config = config.unwrap();

        let config = config
            .lines()
            .enumerate()
            .filter(|&(_, line)| line.len() > 0 && !line.starts_with("#"))
            .collect::<Vec<_>>();

        if config.len() == 0 {

            let err_str = format!(
                "[ Config::build ] The config file does not contain any valid lines!"
            );

            return Err(err_str);
        }

        let mut groups = HashMap::new();

        for (idx, line) in config {

            let split_line = line.split(":").collect::<Vec<_>>();

            if split_line.len() != 2 {

                let err_str = format!(
                    "[ Config::build ] Invalid line syntax at line index {} (line value: \"{}\")", 
                    idx,
                    line
                );

                return Err(err_str);
            }

            let group_name = split_line[0].trim();

            let mut group_values = split_line[1]
                .split(",")
                .map(|x| x.trim())
                .collect::<Vec<_>>();

            if group_values.len() == 2 {

                let group_values = parse_group_values(group_values, idx, line)?;             

                insert_group(
                    &mut groups, 
                    "CT-", 
                    group_name, 
                    IonisableGroup { ka: group_values[0], gco: GroupChargeOptions::NeuOrNeg },
                    idx
                )?;

                insert_group(
                    &mut groups, 
                    "NT-", 
                    group_name, 
                    IonisableGroup { ka: group_values[1], gco: GroupChargeOptions::PosOrNeu }, 
                    idx
                )?;

            } else if group_values.len() == 4 {

                let gco = get_gco(&mut group_values, idx)?;

                let group_values = parse_group_values(group_values, idx, line)?;  

                insert_group(
                    &mut groups, 
                    "CT-", 
                    group_name, 
                    IonisableGroup { ka: group_values[0], gco: GroupChargeOptions::NeuOrNeg }, 
                    idx
                )?;

                insert_group(
                    &mut groups, 
                    "NT-", 
                    group_name, 
                    IonisableGroup { ka: group_values[1], gco: GroupChargeOptions::PosOrNeu }, 
                    idx
                )?;

                insert_group(
                    &mut groups, 
                    "", 
                    group_name, 
                    IonisableGroup { ka: group_values[2], gco }, 
                    idx
                )?;

            } else {

                let err_str = format!(
                    "[ Config::build ] Invalid number of group values at line index {} (line value: \"{}\")", 
                    idx, 
                    line
                );

                return Err(err_str);
            }

        }

        Ok(Self { groups })

    }

    pub fn parse_sequence(&self, sequence: &str) -> Vec<IonisableGroup> {

        let mut output = Vec::new();

        for resi in sequence.as_bytes() {
            let key = String::from(*resi as char);
            if let Some(ig) = self.groups.get(&key) {
                output.push(ig.clone());
            }
        }

        output
    }

}