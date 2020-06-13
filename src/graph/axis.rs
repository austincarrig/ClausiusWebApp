pub enum Direction {
    X,
    Y
}

pub enum ValueType {
    Temperature,
    Pressure,
    SpecificVolume,
    InternalEnergy,
    Enthalpy,
    Entropy
}

pub enum ScaleType {
    Linear,
    Log
}

pub struct Axis {
    pub min: f32,
    pub max: f32,
    pub direction: Direction,
    pub value_type: ValueType,
    pub scale_type: ScaleType
}