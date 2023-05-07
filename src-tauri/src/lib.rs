pub enum Unit {
    Milimeter(f32),
    Centimeter(f32),
    Meter(f32),
    Kilometer(f32),
    Inch(f32),
    Foot(f32),
    Yard(f32),
    Mile(f32),
}

impl Unit {
    pub fn new(num: f32, unit: &String) -> Result<Unit, &'static str> {
        match unit.to_lowercase().as_str().trim() {
            "milimeter" => Ok(Unit::Milimeter(num)),
            "centimeter" => Ok(Unit::Centimeter(num)),
            "meter" => Ok(Unit::Meter(num)),
            "kilometer" => Ok(Unit::Kilometer(num)),
            "inch" => Ok(Unit::Inch(num)),
            "foot" => Ok(Unit::Foot(num)),
            "yard" => Ok(Unit::Yard(num)),
            "mile" => Ok(Unit::Mile(num)),
            _ => Err("Invalid Unit"),
        }
    }
}

pub struct Measurement<'a> {
    pub dimension: &'a str,
    pub uom: Unit,
}

impl Measurement<'_> {
    pub fn to_inch<'a>(self) -> f32 {
        match self.uom {
            Unit::Inch(value) => value.clone(),
            Unit::Milimeter(value) => value.clone()  /25.4,
            Unit::Centimeter(value) => value.clone() /2.54,
            Unit::Meter(value) => value.clone() * 39.37,
            Unit::Kilometer(value) => value.clone() * 39370.0,
            Unit::Foot(value) => value.clone() * 12.0,
            Unit::Yard(value) => value.clone() * 36.0,
            Unit::Mile(value) => value.clone() * 63360.0,
        }
    }
    pub fn to_milimeter<'a>(self) -> f32 {
        match self.uom {
            Unit::Inch(value) => value.clone() * 25.4,
            Unit::Milimeter(value) => value.clone(),
            Unit::Centimeter(value) => value.clone() * 10.0,
            Unit::Meter(value) => value.clone() * 1000.0,
            Unit::Kilometer(value) => value.clone() * 1e6,
            Unit::Foot(value) => value.clone() * 304.8,
            Unit::Yard(value) => value.clone() * 914.4,
            Unit::Mile(value) => value.clone() * 1.609e6,
        }
    }
    pub fn to_centimeter<'a>(self) -> f32 {
        match self.uom {
            Unit::Inch(value) => value.clone() * 2.54,
            Unit::Milimeter(value) => value.clone() / 10.0,
            Unit::Centimeter(value) => value.clone(),
            Unit::Meter(value) => value.clone() * 100.0,
            Unit::Kilometer(value) => value.clone() * 100000.0,
            Unit::Foot(value) => value.clone() * 30.48,
            Unit::Yard(value) => value.clone() * 91.44,
            Unit::Mile(value) => value.clone() * 160900.0,
        }
    }
    pub fn to_meter<'a>(self) -> f32 {
        match self.uom {
            Unit::Inch(value) => value.clone() / 39.37,
            Unit::Milimeter(value) => value.clone() / 1000.0,
            Unit::Centimeter(value) => value.clone() / 100.0,
            Unit::Meter(value) => value.clone(),
            Unit::Kilometer(value) => value.clone() * 1000.0,
            Unit::Foot(value) => value.clone() / 3.281,
            Unit::Yard(value) => value.clone() / 1.094,
            Unit::Mile(value) => value.clone() * 1609.0,
        }
    }
    pub fn to_kilometer<'a>(self) -> f32 {
        match self.uom {
            Unit::Inch(value) => value.clone() / 39370.0,
            Unit::Milimeter(value) => value.clone() / 1e6,
            Unit::Centimeter(value) => value.clone() / 100000.0,
            Unit::Meter(value) => value.clone() / 1000.0,
            Unit::Kilometer(value) => value.clone(),
            Unit::Foot(value) => value.clone() / 3281.0,
            Unit::Yard(value) => value.clone() / 1094.0,
            Unit::Mile(value) => value.clone() * 1.609,
        }
    }
    pub fn to_foot<'a>(self) -> f32 {
        match self.uom {
            Unit::Inch(value) => value.clone() / 12.0,
            Unit::Milimeter(value) => value.clone() / 304.8,
            Unit::Centimeter(value) => value.clone() / 30.48,
            Unit::Meter(value) => value.clone() * 3.281,
            Unit::Kilometer(value) => value.clone() * 3281.0,
            Unit::Foot(value) => value.clone(),
            Unit::Yard(value) => value.clone() * 3.0,
            Unit::Mile(value) => value.clone() * 5280.0,
        }
    }
    pub fn to_yard<'a>(self) -> f32 {
        match self.uom {
            Unit::Inch(value) => value.clone() / 36.0,
            Unit::Milimeter(value) => value.clone() / 914.4,
            Unit::Centimeter(value) => value.clone() / 91.44,
            Unit::Meter(value) => value.clone() * 1.094,
            Unit::Kilometer(value) => value.clone() * 1094.0,
            Unit::Foot(value) => value.clone() / 3.0,
            Unit::Yard(value) => value.clone(),
            Unit::Mile(value) => value.clone() * 1760.0,
        }
    }
    pub fn to_mile<'a>(self) -> f32 {
        match self.uom {
            Unit::Inch(value) => value.clone() / 63360.0,
            Unit::Milimeter(value) => value.clone() / 1.609e6,
            Unit::Centimeter(value) => value.clone() / 160900.0,
            Unit::Meter(value) => value.clone() / 1609.0,
            Unit::Kilometer(value) => value.clone() / 1.609,
            Unit::Foot(value) => value.clone() / 5280.0,
            Unit::Yard(value) => value.clone() / 1760.0,
            Unit::Mile(value) => value.clone(),
        }
    }

    pub fn to_specific<'a>(self, uom: &String) -> Result<f32,&'static str> {
        match uom.to_lowercase().as_str().trim() {
            "milimeter" => Ok(self.to_milimeter()),
            "centimeter" => Ok(self.to_centimeter()),
            "meter" => Ok(self.to_meter()),
            "kilometer" => Ok(self.to_kilometer()),
            "inch" => Ok(self.to_inch()),
            "foot" => Ok(self.to_foot()),
            "yard" => Ok(self.to_yard()),
            "mile" => Ok(self.to_mile()),
            _ => Err("Invalid Unit"),
        }
    }
}