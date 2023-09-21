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

    pub fn to_inch<'a>(self) -> f32 {
        match &self {
            Distance::Inch(value) => value.to_owned(),
            Distance::Milimeter(value) => value / 25.4,
            Distance::Centimeter(value) => value / 2.54,
            Distance::Meter(value) => value * 39.37,
            Distance::Kilometer(value) => value * 39370.0,
            Distance::Foot(value) => value * 12.0,
            Distance::Yard(value) => value * 36.0,
            Distance::Mile(value) => value * 63360.0,
        }
    }
    pub fn to_milimeter<'a>(self) -> f32 {
        match &self {
            Distance::Inch(value) => value * 25.4,
            Distance::Milimeter(value) => value.to_owned(),
            Distance::Centimeter(value) => value * 10.0,
            Distance::Meter(value) => value * 1000.0,
            Distance::Kilometer(value) => value * 1e6,
            Distance::Foot(value) => value * 304.8,
            Distance::Yard(value) => value * 914.4,
            Distance::Mile(value) => value * 1.609e6,
        }
    }
    pub fn to_centimeter<'a>(self) -> f32 {
        match &self {
            Distance::Inch(value) => value * 2.54,
            Distance::Milimeter(value) => value / 10.0,
            Distance::Centimeter(value) => value.to_owned(),
            Distance::Meter(value) => value * 100.0,
            Distance::Kilometer(value) => value * 100000.0,
            Distance::Foot(value) => value * 30.48,
            Distance::Yard(value) => value * 91.44,
            Distance::Mile(value) => value * 160900.0,
        }
    }
    pub fn to_meter<'a>(self) -> f32 {
        match &self {
            Distance::Inch(value) => value / 39.37,
            Distance::Milimeter(value) => value / 1000.0,
            Distance::Centimeter(value) => value / 100.0,
            Distance::Meter(value) => value.to_owned(),
            Distance::Kilometer(value) => value * 1000.0,
            Distance::Foot(value) => value / 3.281,
            Distance::Yard(value) => value / 1.094,
            Distance::Mile(value) => value * 1609.0,
        }
    }
    pub fn to_kilometer<'a>(self) -> f32 {
        match &self {
            Distance::Inch(value) => value / 39370.0,
            Distance::Milimeter(value) => value / 1e6,
            Distance::Centimeter(value) => value / 100000.0,
            Distance::Meter(value) => value / 1000.0,
            Distance::Kilometer(value) => value.to_owned(),
            Distance::Foot(value) => value / 3281.0,
            Distance::Yard(value) => value / 1094.0,
            Distance::Mile(value) => value * 1.609,
        }
    }
    pub fn to_foot<'a>(self) -> f32 {
        match &self {
            Distance::Inch(value) => value / 12.0,
            Distance::Milimeter(value) => value / 304.8,
            Distance::Centimeter(value) => value / 30.48,
            Distance::Meter(value) => value * 3.281,
            Distance::Kilometer(value) => value * 3281.0,
            Distance::Foot(value) => value.to_owned(),
            Distance::Yard(value) => value * 3.0,
            Distance::Mile(value) => value * 5280.0,
        }
    }
    pub fn to_yard<'a>(self) -> f32 {
        match &self {
            Distance::Inch(value) => value / 36.0,
            Distance::Milimeter(value) => value / 914.4,
            Distance::Centimeter(value) => value / 91.44,
            Distance::Meter(value) => value * 1.094,
            Distance::Kilometer(value) => value * 1094.0,
            Distance::Foot(value) => value / 3.0,
            Distance::Yard(value) => value.to_owned(),
            Distance::Mile(value) => value * 1760.0,
        }
    }
    pub fn to_mile<'a>(self) -> f32 {
        match &self {
            Distance::Inch(value) => value / 63360.0,
            Distance::Milimeter(value) => value / 1.609e6,
            Distance::Centimeter(value) => value / 160900.0,
            Distance::Meter(value) => value / 1609.0,
            Distance::Kilometer(value) => value / 1.609,
            Distance::Foot(value) => value / 5280.0,
            Distance::Yard(value) => value / 1760.0,
            Distance::Mile(value) => value.to_owned(),
        }
    }

    pub fn to_specific<'a>(self, uom: &String) -> Result<f32, &'static str> {
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

pub enum Volume {
    Gallon(f32),
    Quart(f32),
    Pint(f32),
    Cup(f32),
    Ounce(f32),
    Tablespoon(f32),
    Teaspoon(f32),
    Liter(f32),
    Milliliter(f32),
    CubicMeter(f32),
    CubicFoot(f32),
    CubitInch(f32),
}


impl Volume {
    pub fn new(num: f32, unit: &String) -> Result<Volume, &'static str> {
        match unit.to_lowercase().as_str().trim() {
            "gallon" => Ok(Volume::Gallon(num)),
            "quart" => Ok(Volume::Quart(num)),
            "pint" => Ok(Volume::Pint(num)),
            "cup" => Ok(Volume::Cup(num)),
            "ounce" => Ok(Volume::Ounce(num)),
            "tablespoon" => Ok(Volume::Tablespoon(num)),
            "teaspoon" => Ok(Volume::Teaspoon(num)),
            "liter" => Ok(Volume::Liter(num)),
            "milliliter"=> Ok(Volume::Milliliter(num)),
            "cubic meter" => Ok(Volume::CubicMeter(num)),
            "cubic foot" => Ok(Volume::CubicFoot(num)),
            "cubic inch" => Ok(Volume::CubitInch(num)),
            _ => Err("Invalid Volume"),
        }
    }
    pub fn to_gallon<'a>(self) -> f32 {
        match &self {
            Volume::Gallon(value) => value.to_owned(),
            Volume::Quart(value) => value / 4.0,
            Volume::Pint(value) => value / 8.0,
            Volume::Cup(value) => value / 15.772,
            Volume::Ounce(value) => value / 128.0,
            Volume::Tablespoon(value) => value / 256.0,
            Volume::Teaspoon(value) => value / 768.0,
            Volume::Liter(value) => value / 3.785,
            Volume::Milliliter(value) => value / 3785.0,
            Volume::CubicMeter(value) => value / 264.2,
            Volume::CubicFoot(value) => value / 7.481,
            Volume::CubitInch(value) => value / 231.0,

        }
    }    
    pub fn to_quart<'a>(self) -> f32 {
        match &self {
            Volume::Gallon(value) => value * 4.0,
            Volume::Quart(value) => value.to_owned(),
            Volume::Pint(value) => value / 2.0,
            Volume::Cup(value) => value / 3.943,
            Volume::Ounce(value) => value / 32.0,
            Volume::Tablespoon(value) => value / 64.0,
            Volume::Teaspoon(value) => value / 192.0,
            Volume::Liter(value) => value * 1.057,
            Volume::Milliliter(value) => value / 946.4,
            Volume::CubicMeter(value) => value * 1057.0,
            Volume::CubicFoot(value) => value * 29.922,
            Volume::CubitInch(value) => value / 57.75,

        }
    }   
    pub fn to_pint<'a>(self) -> f32 {
        match &self {
            Volume::Gallon(value) => value * 8.0,
            Volume::Quart(value) => value* 2.0,
            Volume::Pint(value) => value.to_owned(),
            Volume::Cup(value) => value / 1.972,
            Volume::Ounce(value) => value / 16.0,
            Volume::Tablespoon(value) => value / 32.0,
            Volume::Teaspoon(value) => value / 96.0,
            Volume::Liter(value) => value * 2.113,
            Volume::Milliliter(value) => value / 473.2,
            Volume::CubicMeter(value) => value * 2113.0,
            Volume::CubicFoot(value) => value * 59.844,
            Volume::CubitInch(value) => value / 28.875,

        }
    }
    pub fn to_cup<'a>(self) -> f32 {
        match &self {
            Volume::Gallon(value) => value * 15.773,
            Volume::Quart(value) => value* 3.943,
            Volume::Pint(value) => value* 1.972,
            Volume::Cup(value) => value.to_owned(),
            Volume::Ounce(value) => value / 8.115,
            Volume::Tablespoon(value) => value / 16.231,
            Volume::Teaspoon(value) => value / 48.692,
            Volume::Liter(value) => value * 4.167,
            Volume::Milliliter(value) => value / 240.0,
            Volume::CubicMeter(value) => value * 4167.0,
            Volume::CubicFoot(value) => value * 118.0,
            Volume::CubitInch(value) => value / 14.646,

        }
    }
    pub fn to_ounce<'a>(self) -> f32 {
        match &self {
            Volume::Gallon(value) => value * 128.0,
            Volume::Quart(value) => value* 32.0,
            Volume::Pint(value) => value* 16.0,
            Volume::Cup(value) => value * 8.115,
            Volume::Ounce(value) => value.to_owned(),
            Volume::Tablespoon(value) => value / 2.0,
            Volume::Teaspoon(value) => value / 6.0,
            Volume::Liter(value) => value * 33.81,
            Volume::Milliliter(value) => value / 29.574,
            Volume::CubicMeter(value) => value * 33810.0,
            Volume::CubicFoot(value) => value * 957.5,
            Volume::CubitInch(value) => value / 1.805,

        }
    }
    pub fn to_tablespoon<'a>(self) -> f32 {
        match &self {
            Volume::Gallon(value) => value * 256.0,
            Volume::Quart(value) => value* 64.0,
            Volume::Pint(value) => value* 32.0,
            Volume::Cup(value) => value * 16.231,
            Volume::Ounce(value) => value * 2.0,
            Volume::Tablespoon(value) => value.to_owned(),
            Volume::Teaspoon(value) => value / 3.0,
            Volume::Liter(value) => value * 67.628,
            Volume::Milliliter(value) => value / 14.787,
            Volume::CubicMeter(value) => value * 67630.0,
            Volume::CubicFoot(value) => value * 1915.0,
            Volume::CubitInch(value) => value * 1.108,

        }
    }
    pub fn to_teaspoon<'a>(self) -> f32 {
        match &self {
            Volume::Gallon(value) => value * 768.0,
            Volume::Quart(value) => value* 192.0,
            Volume::Pint(value) => value* 96.0,
            Volume::Cup(value) => value * 48.692,
            Volume::Ounce(value) => value * 6.0,
            Volume::Tablespoon(value) => value *3.0,
            Volume::Teaspoon(value) => value.to_owned(),
            Volume::Liter(value) => value * 202.9,
            Volume::Milliliter(value) => value / 4.929,
            Volume::CubicMeter(value) => value * 202900.0,
            Volume::CubicFoot(value) => value * 5745.0,
            Volume::CubitInch(value) => value * 3.325,

        }
    }
    pub fn to_liter<'a>(self) -> f32 {
        match &self {
            Volume::Gallon(value) => value / 3.785,
            Volume::Quart(value) => value* 1.057,
            Volume::Pint(value) => value/ 2.113,
            Volume::Cup(value) => value / 4.167,
            Volume::Ounce(value) => value / 33.814,
            Volume::Tablespoon(value) => value / 67.628,
            Volume::Teaspoon(value) => value / 202.9 ,
            Volume::Liter(value) => value.to_owned(),
            Volume::Milliliter(value) => value / 1000.0,
            Volume::CubicMeter(value) => value * 1000.0,
            Volume::CubicFoot(value) => value * 28.317,
            Volume::CubitInch(value) => value / 61.024,

        }
    }
    pub fn to_milliliter<'a>(self) -> f32 {
        match &self {
            Volume::Gallon(value) => value * 3785.0,
            Volume::Quart(value) => value * 946.4,
            Volume::Pint(value) => value * 473.2,
            Volume::Cup(value) => value * 240.0,
            Volume::Ounce(value) => value * 29.574,
            Volume::Tablespoon(value) => value * 14.787,
            Volume::Teaspoon(value) => value * 4.929 ,
            Volume::Liter(value) => value * 1000.0,
            Volume::Milliliter(value) => value.to_owned(),
            Volume::CubicMeter(value) => value * 1e+6,
            Volume::CubicFoot(value) => value * 28320.0,
            Volume::CubitInch(value) => value * 16.387,

        }
    }
    pub fn to_cubic_meter<'a>(self) -> f32 {
        match &self {
            Volume::Gallon(value) => value / 264.2,
            Volume::Quart(value) => value / 1057.0,
            Volume::Pint(value) => value / 2113.0,
            Volume::Cup(value) => value / 4167.0,
            Volume::Ounce(value) => value / 33810.0,
            Volume::Tablespoon(value) => value / 67630.0,
            Volume::Teaspoon(value) => value / 202900.0 ,
            Volume::Liter(value) => value / 1000.0,
            Volume::Milliliter(value) => value / 1e+6,
            Volume::CubicMeter(value) => value.to_owned(),
            Volume::CubicFoot(value) => value / 35.315,
            Volume::CubitInch(value) => value / 61020.0,

        }
    }
    pub fn to_cubic_foot<'a>(self) -> f32 {
        match &self {
            Volume::Gallon(value) => value / 7.48,
            Volume::Quart(value) => value / 29.922,
            Volume::Pint(value) => value / 59.844,
            Volume::Cup(value) => value / 118.0,
            Volume::Ounce(value) => value / 957.5,
            Volume::Tablespoon(value) => value / 1915.0,
            Volume::Teaspoon(value) => value / 5745.0 ,
            Volume::Liter(value) => value / 28.317,
            Volume::Milliliter(value) => value / 28320.0,
            Volume::CubicMeter(value) => value * 35.315,
            Volume::CubicFoot(value) => value.to_owned(),
            Volume::CubitInch(value) => value / 1728.0,

        }
    }
    pub fn to_cubic_inch<'a>(self) -> f32 {
        match &self {
            Volume::Gallon(value) => value * 231.0,
            Volume::Quart(value) => value * 57.75,
            Volume::Pint(value) => value * 28.875,
            Volume::Cup(value) => value * 14.646,
            Volume::Ounce(value) => value * 1.805,
            Volume::Tablespoon(value) => value / 1.108,
            Volume::Teaspoon(value) => value / 3.325 ,
            Volume::Liter(value) => value * 61.024,
            Volume::Milliliter(value) => value / 16.387,
            Volume::CubicMeter(value) => value * 61020.0,
            Volume::CubicFoot(value) => value * 1728.0,
            Volume::CubitInch(value) => value.to_owned(),

        }
    }
    pub fn to_specific<'a>(self, uom: &String) -> Result<f32, &'static str> {
        match uom.to_lowercase().as_str().trim() {
            "gallon" => Ok(self.to_gallon()),
            "quart" => Ok(self.to_quart()),
            "pint" => Ok(self.to_pint()),
            "cup" => Ok(self.to_cup()),
            "ounce" => Ok(self.to_ounce()),
            "tablespoon" => Ok(self.to_tablespoon()),
            "teaspoon" => Ok(self.to_teaspoon()),
            "liter" => Ok(self.to_liter()),
            "milliliter" => Ok(self.to_milliliter()),
            "cubic meter" => Ok(self.to_cubic_meter()),
            "cubic foot" => Ok(self.to_cubic_foot()),
            "cubic inch" => Ok(self.to_cubic_inch()),
            _ => Err("Invalid  unit of measurement."),
        }
    }
}




