pub enum Unit {
    Mililliter(f32),
    Liter(f32),
    Milimeter(f32),
    Centimeter(f32),
    Meter(f32),
    Kilometer(f32),
    KPH(f32),
    Inch(f32),
    Foot(f32),
    Yard(f32),
    Mile(f32),
    Ounce(f32),
    Quart(f32),
    Pint(f32),
    Gallon(f32),
    MPH(f32),
    LPH(f32),
    GPH(f32),
}

impl Unit {
    pub fn new(num: f32, unit: &String) -> Result<Unit, &'static str> {
        match unit.to_lowercase().as_str().trim() {
            "mililliter" => Ok(Unit::Mililliter(num)),
            "liter" => Ok(Unit::Liter(num)),
            "milimeter" => Ok(Unit::Milimeter(num)),
            "centimeter" => Ok(Unit::Centimeter(num)),
            "meter" => Ok(Unit::Meter(num)),
            "kilometer" => Ok(Unit::Kilometer(num)),
            "kph" => Ok(Unit::KPH(num)),
            "inch" => Ok(Unit::Inch(num)),
            "foot" => Ok(Unit::Foot(num)),
            "yard" => Ok(Unit::Yard(num)),
            "mile" => Ok(Unit::Mile(num)),
            "ounce" => Ok(Unit::Ounce(num)),
            "quart" => Ok(Unit::Quart(num)),
            "pint" => Ok(Unit::Gallon(num)),
            "mph" => Ok(Unit::MPH(num)),
            "lph" => Ok(Unit::LPH(num)),
            "gph" => Ok(Unit::GPH(num)),
            _ => Err("Invalid Unit"),
        }
    }
}

pub struct Measurement<'a> {
   pub dimension: &'a str,
   pub uom: Unit
}

impl Measurement<'_> {
   

pub fn to_inch<'a>(self) -> Result<f32, &'static str> {
    match self.uom {
        Unit::Inch(value) => Ok(value.clone()),
        Unit::Milimeter(value) => Ok(value.clone() * 25.4),
        Unit::Centimeter(value) => Ok(value.to_owned() * 2.54),
        Unit::Meter(value) => Ok(value.to_owned() * 39.37),
        Unit::Kilometer(value) => Ok(value.to_owned() * 39370.0),
        Unit::Foot(value) => Ok(value.to_owned() / 12.0),
        Unit::Yard(value) => Ok(value.to_owned() * 36.0),
        Unit::Mile(value) => Ok(value.to_owned() * 63360.0),
        _ => Err("Unable to convert measurement."),
    }
}
pub fn to_milimeter<'a>(self) -> Result<f32, &'static str> {
      match self.uom {
        Unit::Inch(value) => Ok(value.clone() * 25.4),
        Unit::Milimeter(value) => Ok(value.clone()),
        Unit::Centimeter(value) => Ok(value.to_owned() * 10.0),
        Unit::Meter(value) => Ok(value.to_owned() * 1000.0),
        Unit::Kilometer(value) => Ok(value.to_owned() * 1e6),
        Unit::Foot(value) => Ok(value.to_owned() * 304.8),
        Unit::Yard(value) => Ok(value.to_owned() * 914.4),
        Unit::Mile(value) => Ok(value.to_owned() * 1.609e6),
        _ => Err("Unable to convert measurement."),
    }
}
pub fn to_centimeter<'a>(self) -> Result<f32, &'static str> {
      match self.uom {
        Unit::Inch(value) => Ok(value.clone() * 2.54),
        Unit::Milimeter(value) => Ok(value.clone() / 10.0),
        Unit::Centimeter(value) => Ok(value.to_owned()),
        Unit::Meter(value) => Ok(value.to_owned() * 100.0),
        Unit::Kilometer(value) => Ok(value.to_owned() * 100000.0),
        Unit::Foot(value) => Ok(value.to_owned() * 30.48),
        Unit::Yard(value) => Ok(value.to_owned() * 91.44),
        Unit::Mile(value) => Ok(value.to_owned() * 160900.0),
        _ => Err("Unable to convert measurement."),
    }
}
pub fn to_meter<'a>(self) -> Result<f32, &'static str> {
      match self.uom {
        Unit::Inch(value) => Ok(value.clone() / 39.37),
        Unit::Milimeter(value) => Ok(value.clone() / 1000.0),
        Unit::Centimeter(value) => Ok(value.to_owned() / 100.0),
        Unit::Meter(value) => Ok(value.to_owned()),
        Unit::Kilometer(value) => Ok(value.to_owned() * 1000.0),
        Unit::Foot(value) => Ok(value.to_owned() / 3.281),
        Unit::Yard(value) => Ok(value.to_owned() / 1.094),
        Unit::Mile(value) => Ok(value.to_owned() * 1609.0),
        _ => Err("Unable to convert measurement."),
    }
}
pub fn to_kilometer<'a>(self) -> Result<f32, &'static str> {
      match self.uom {
        Unit::Inch(value) => Ok(value.clone() / 39370.0),
        Unit::Milimeter(value) => Ok(value.clone() / 1e6),
        Unit::Centimeter(value) => Ok(value.to_owned() / 100000.0),
        Unit::Meter(value) => Ok(value.to_owned() / 1000.0),
        Unit::Kilometer(value) => Ok(value.to_owned()),
        Unit::Foot(value) => Ok(value.to_owned() / 3281.0),
        Unit::Yard(value) => Ok(value.to_owned() / 1094.0),
        Unit::Mile(value) => Ok(value.to_owned() * 1.609),
        _ => Err("Unable to convert measurement."),
    }
}
pub fn to_foot<'a>(self) -> Result<f32, &'static str> {
      match self.uom {
        Unit::Inch(value) => Ok(value.clone() / 12.0),
        Unit::Milimeter(value) => Ok(value.clone() / 304.8),
        Unit::Centimeter(value) => Ok(value.to_owned() / 30.48),
        Unit::Meter(value) => Ok(value.to_owned() * 3.281),
        Unit::Kilometer(value) => Ok(value.to_owned() * 3281.0),
        Unit::Foot(value) => Ok(value.to_owned()),
        Unit::Yard(value) => Ok(value.to_owned() / 3.0),
        Unit::Mile(value) => Ok(value.to_owned() * 5280.0),
        _ => Err("Unable to convert measurement."),
    }
}
pub fn to_yard<'a>(self) -> Result<f32, &'static str> {
      match self.uom {
        Unit::Inch(value) => Ok(value.clone() / 36.0),
        Unit::Milimeter(value) => Ok(value.clone() / 914.4),
        Unit::Centimeter(value) => Ok(value.to_owned() / 91.44),
        Unit::Meter(value) => Ok(value.to_owned() * 1.094),
        Unit::Kilometer(value) => Ok(value.to_owned() * 1094.0),
        Unit::Foot(value) => Ok(value.to_owned() / 3.0),
        Unit::Yard(value) => Ok(value.to_owned()),
        Unit::Mile(value) => Ok(value.to_owned() * 1760.0),
        _ => Err("Unable to convert measurement."),
    }
}
pub fn to_mile<'a>(self) -> Result<f32, &'static str> {
      match self.uom {
        Unit::Inch(value) => Ok(value.clone() / 63360.0),
        Unit::Milimeter(value) => Ok(value.clone() / 1.609e6),
        Unit::Centimeter(value) => Ok(value.to_owned() / 160900.0),
        Unit::Meter(value) => Ok(value.to_owned() / 1609.0),
        Unit::Kilometer(value) => Ok(value.to_owned() / 1.609),
        Unit::Foot(value) => Ok(value.to_owned() / 5280.0),
        Unit::Yard(value) => Ok(value.to_owned() / 1760.0),
        Unit::Mile(value) => Ok(value.to_owned()),
        _ => Err("Unable to convert measurement."),
    }
}}