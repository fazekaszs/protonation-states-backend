#[derive(Clone, Debug)]
pub enum GroupChargeOptions {
    PosOrNeu, NeuOrNeg
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

/// Stores a charge microstate with a certain charge pattern
/// (e.g. \[+1, 0, 0, -1, 0, -1, +1\])
/// and with a certain prevalence
/// (what fraction of the total concentration this microstate represents).
#[derive(Clone, Debug)]
pub struct MicroState {
    pub prevalence: f32,
    pub charge_pattern: Vec<i32>
}