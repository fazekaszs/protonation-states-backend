/// Distinguishes the two charge state types of amino acids.
#[derive(Clone, Debug)]
pub enum GroupChargeOptions {
    PosOrNeu, NeuOrNeg
}

impl GroupChargeOptions {

    pub fn from_str(arg: &str) -> Result<Self, String> {
        match arg {
            "PosOrNeu" => Ok(Self::PosOrNeu),
            "NeuOrNeg" => Ok(Self::NeuOrNeg),
            _ => Err(format!("Unable to deserialize \"{}\" to GroupChargeOptions", arg))
        }
    }

    pub fn to_tuple(&self) -> (i32, i32) {

        match self { 
            GroupChargeOptions::PosOrNeu => (1, 0), 
            GroupChargeOptions::NeuOrNeg => (0, -1) 
        }
    }
}

/// Represents an ionisable group with a certain Ka
/// (acid dissociation constant)
/// and a certain group charge option
/// (what charge it takes at low pH (+1 or 0) and at high pH (0 or -1)).
#[derive(Clone, Debug)]
pub struct IonisableGroup {
    pub ka: f32,
    pub gco: GroupChargeOptions
}

impl IonisableGroup {

    pub fn from_str(arg: &str) -> Result<Self, String> {

        let elements = arg.split(",").map(|x| x.trim()).collect::<Vec<_>>();

        if elements.len() != 2 { 
            let err_str = format!("Unable to parse \"{}\"! More than two elements detected!", arg);
            return Err(err_str);
        }

        let gco = GroupChargeOptions::from_str(elements[0])?;

        let Ok(pka) = elements[1].parse::<f32>() else {

            let err_str = format!("Unable to parse \"{}\"! Second element is not a valid number!", arg);
            return Err(err_str);            
        };

        Ok(IonisableGroup { ka: 10f32.powf(-pka), gco })

    }

}

/// Stores a charge microstate with a certain charge pattern
/// (e.g. \[+1, 0, 0, -1, 0, -1, +1\])
/// and with a certain prevalence
/// (what fraction of the total concentration this microstate represents).
#[derive(Clone, Debug)]
pub struct MicroState {
    pub prevalence: f32,
    pub charge_pattern: Vec<i32>
}