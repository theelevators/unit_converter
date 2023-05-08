pub enum Distance {
    Milimeter(f32),
    Centimeter(f32),
    Meter(f32),
    Kilometer(f32),
    Inch(f32),
    Foot(f32),
    Yard(f32),
    Mile(f32),
}

impl Distance {
    pub fn new(num: f32, unit: &String) -> Result<Distance, &'static str> {
        match unit.to_lowercase().as_str().trim() {
            "milimeter" => Ok(Distance::Milimeter(num)),
            "centimeter" => Ok(Distance::Centimeter(num)),
            "meter" => Ok(Distance::Meter(num)),
            "kilometer" => Ok(Distance::Kilometer(num)),
            "inch" => Ok(Distance::Inch(num)),
            "foot" => Ok(Distance::Foot(num)),
            "yard" => Ok(Distance::Yard(num)),
            "mile" => Ok(Distance::Mile(num)),
            _ => Err("Invalid Distance"),
        }
    }
}

pub struct Measurement<'a> {
    pub dimension: &'a str,
    pub uom: Distance,
}

impl Measurement<'_> {
    pub fn to_inch<'a>(self) -> f32 {
        match self.uom {
            Distance::Inch(value) => value.clone(),
            Distance::Milimeter(value) => value.clone()  /25.4,
            Distance::Centimeter(value) => value.clone() /2.54,
            Distance::Meter(value) => value.clone() * 39.37,
            Distance::Kilometer(value) => value.clone() * 39370.0,
            Distance::Foot(value) => value.clone() * 12.0,
            Distance::Yard(value) => value.clone() * 36.0,
            Distance::Mile(value) => value.clone() * 63360.0,
        }
    }
    pub fn to_milimeter<'a>(self) -> f32 {
        match self.uom {
            Distance::Inch(value) => value.clone() * 25.4,
            Distance::Milimeter(value) => value.clone(),
            Distance::Centimeter(value) => value.clone() * 10.0,
            Distance::Meter(value) => value.clone() * 1000.0,
            Distance::Kilometer(value) => value.clone() * 1e6,
            Distance::Foot(value) => value.clone() * 304.8,
            Distance::Yard(value) => value.clone() * 914.4,
            Distance::Mile(value) => value.clone() * 1.609e6,
        }
    }
    pub fn to_centimeter<'a>(self) -> f32 {
        match self.uom {
            Distance::Inch(value) => value.clone() * 2.54,
            Distance::Milimeter(value) => value.clone() / 10.0,
            Distance::Centimeter(value) => value.clone(),
            Distance::Meter(value) => value.clone() * 100.0,
            Distance::Kilometer(value) => value.clone() * 100000.0,
            Distance::Foot(value) => value.clone() * 30.48,
            Distance::Yard(value) => value.clone() * 91.44,
            Distance::Mile(value) => value.clone() * 160900.0,
        }
    }
    pub fn to_meter<'a>(self) -> f32 {
        match self.uom {
            Distance::Inch(value) => value.clone() / 39.37,
            Distance::Milimeter(value) => value.clone() / 1000.0,
            Distance::Centimeter(value) => value.clone() / 100.0,
            Distance::Meter(value) => value.clone(),
            Distance::Kilometer(value) => value.clone() * 1000.0,
            Distance::Foot(value) => value.clone() / 3.281,
            Distance::Yard(value) => value.clone() / 1.094,
            Distance::Mile(value) => value.clone() * 1609.0,
        }
    }
    pub fn to_kilometer<'a>(self) -> f32 {
        match self.uom {
            Distance::Inch(value) => value.clone() / 39370.0,
            Distance::Milimeter(value) => value.clone() / 1e6,
            Distance::Centimeter(value) => value.clone() / 100000.0,
            Distance::Meter(value) => value.clone() / 1000.0,
            Distance::Kilometer(value) => value.clone(),
            Distance::Foot(value) => value.clone() / 3281.0,
            Distance::Yard(value) => value.clone() / 1094.0,
            Distance::Mile(value) => value.clone() * 1.609,
        }
    }
    pub fn to_foot<'a>(self) -> f32 {
        match self.uom {
            Distance::Inch(value) => value.clone() / 12.0,
            Distance::Milimeter(value) => value.clone() / 304.8,
            Distance::Centimeter(value) => value.clone() / 30.48,
            Distance::Meter(value) => value.clone() * 3.281,
            Distance::Kilometer(value) => value.clone() * 3281.0,
            Distance::Foot(value) => value.clone(),
            Distance::Yard(value) => value.clone() * 3.0,
            Distance::Mile(value) => value.clone() * 5280.0,
        }
    }
    pub fn to_yard<'a>(self) -> f32 {
        match self.uom {
            Distance::Inch(value) => value.clone() / 36.0,
            Distance::Milimeter(value) => value.clone() / 914.4,
            Distance::Centimeter(value) => value.clone() / 91.44,
            Distance::Meter(value) => value.clone() * 1.094,
            Distance::Kilometer(value) => value.clone() * 1094.0,
            Distance::Foot(value) => value.clone() / 3.0,
            Distance::Yard(value) => value.clone(),
            Distance::Mile(value) => value.clone() * 1760.0,
        }
    }
    pub fn to_mile<'a>(self) -> f32 {
        match self.uom {
            Distance::Inch(value) => value.clone() / 63360.0,
            Distance::Milimeter(value) => value.clone() / 1.609e6,
            Distance::Centimeter(value) => value.clone() / 160900.0,
            Distance::Meter(value) => value.clone() / 1609.0,
            Distance::Kilometer(value) => value.clone() / 1.609,
            Distance::Foot(value) => value.clone() / 5280.0,
            Distance::Yard(value) => value.clone() / 1760.0,
            Distance::Mile(value) => value.clone(),
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
            _ => Err("Invalid  unit of measurement."),
        }
    }
}