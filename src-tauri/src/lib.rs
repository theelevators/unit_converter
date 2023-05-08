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
        match self {
            Distance::Inch(value) => value.clone(),
            Distance::Milimeter(value) => value.clone() / 25.4,
            Distance::Centimeter(value) => value.clone() / 2.54,
            Distance::Meter(value) => value.clone() * 39.37,
            Distance::Kilometer(value) => value.clone() * 39370.0,
            Distance::Foot(value) => value.clone() * 12.0,
            Distance::Yard(value) => value.clone() * 36.0,
            Distance::Mile(value) => value.clone() * 63360.0,
        }
    }
    pub fn to_milimeter<'a>(self) -> f32 {
        match self {
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
        match self {
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
        match self {
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
        match self {
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
        match self {
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
        match self {
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
        match self {
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
        match self {
            Volume::Gallon(value) => value.clone(),
            Volume::Quart(value) => value.clone() / 4.0,
            Volume::Pint(value) => value.clone() / 8.0,
            Volume::Cup(value) => value.clone() / 15.772,
            Volume::Ounce(value) => value.clone() / 128.0,
            Volume::Tablespoon(value) => value.clone() / 256.0,
            Volume::Teaspoon(value) => value.clone() / 768.0,
            Volume::Liter(value) => value.clone() / 3.785,
            Volume::Milliliter(value) => value.clone() / 3785.0,
            Volume::CubicMeter(value) => value.clone() / 264.2,
            Volume::CubicFoot(value) => value.clone() / 7.481,
            Volume::CubitInch(value) => value.clone() / 231.0,

        }
    }    
    pub fn to_quart<'a>(self) -> f32 {
        match self {
            Volume::Gallon(value) => value.clone() * 4.0,
            Volume::Quart(value) => value.clone(),
            Volume::Pint(value) => value.clone() / 2.0,
            Volume::Cup(value) => value.clone() / 3.943,
            Volume::Ounce(value) => value.clone() / 32.0,
            Volume::Tablespoon(value) => value.clone() / 64.0,
            Volume::Teaspoon(value) => value.clone() / 192.0,
            Volume::Liter(value) => value.clone() * 1.057,
            Volume::Milliliter(value) => value.clone() / 946.4,
            Volume::CubicMeter(value) => value.clone() * 1057.0,
            Volume::CubicFoot(value) => value.clone() * 29.922,
            Volume::CubitInch(value) => value.clone() / 57.75,

        }
    }   
    pub fn to_pint<'a>(self) -> f32 {
        match self {
            Volume::Gallon(value) => value.clone() * 8.0,
            Volume::Quart(value) => value.clone()* 2.0,
            Volume::Pint(value) => value.clone(),
            Volume::Cup(value) => value.clone() / 1.972,
            Volume::Ounce(value) => value.clone() / 16.0,
            Volume::Tablespoon(value) => value.clone() / 32.0,
            Volume::Teaspoon(value) => value.clone() / 96.0,
            Volume::Liter(value) => value.clone() * 2.113,
            Volume::Milliliter(value) => value.clone() / 473.2,
            Volume::CubicMeter(value) => value.clone() * 2113.0,
            Volume::CubicFoot(value) => value.clone() * 59.844,
            Volume::CubitInch(value) => value.clone() / 28.875,

        }
    }
    pub fn to_cup<'a>(self) -> f32 {
        match self {
            Volume::Gallon(value) => value.clone() * 15.773,
            Volume::Quart(value) => value.clone()* 3.943,
            Volume::Pint(value) => value.clone()* 1.972,
            Volume::Cup(value) => value.clone() ,
            Volume::Ounce(value) => value.clone() / 8.115,
            Volume::Tablespoon(value) => value.clone() / 16.231,
            Volume::Teaspoon(value) => value.clone() / 48.692,
            Volume::Liter(value) => value.clone() * 4.167,
            Volume::Milliliter(value) => value.clone() / 240.0,
            Volume::CubicMeter(value) => value.clone() * 4167.0,
            Volume::CubicFoot(value) => value.clone() * 118.0,
            Volume::CubitInch(value) => value.clone() / 14.646,

        }
    }
    pub fn to_ounce<'a>(self) -> f32 {
        match self {
            Volume::Gallon(value) => value.clone() * 128.0,
            Volume::Quart(value) => value.clone()* 32.0,
            Volume::Pint(value) => value.clone()* 16.0,
            Volume::Cup(value) => value.clone() * 8.115,
            Volume::Ounce(value) => value.clone(),
            Volume::Tablespoon(value) => value.clone() / 2.0,
            Volume::Teaspoon(value) => value.clone() / 6.0,
            Volume::Liter(value) => value.clone() * 33.81,
            Volume::Milliliter(value) => value.clone() / 29.574,
            Volume::CubicMeter(value) => value.clone() * 33810.0,
            Volume::CubicFoot(value) => value.clone() * 957.5,
            Volume::CubitInch(value) => value.clone() / 1.805,

        }
    }
    pub fn to_tablespoon<'a>(self) -> f32 {
        match self {
            Volume::Gallon(value) => value.clone() * 256.0,
            Volume::Quart(value) => value.clone()* 64.0,
            Volume::Pint(value) => value.clone()* 32.0,
            Volume::Cup(value) => value.clone() * 16.231,
            Volume::Ounce(value) => value.clone() * 2.0,
            Volume::Tablespoon(value) => value.clone() ,
            Volume::Teaspoon(value) => value.clone() / 3.0,
            Volume::Liter(value) => value.clone() * 67.628,
            Volume::Milliliter(value) => value.clone() / 14.787,
            Volume::CubicMeter(value) => value.clone() * 67630.0,
            Volume::CubicFoot(value) => value.clone() * 1915.0,
            Volume::CubitInch(value) => value.clone() * 1.108,

        }
    }
    pub fn to_teaspoon<'a>(self) -> f32 {
        match self {
            Volume::Gallon(value) => value.clone() * 768.0,
            Volume::Quart(value) => value.clone()* 192.0,
            Volume::Pint(value) => value.clone()* 96.0,
            Volume::Cup(value) => value.clone() * 48.692,
            Volume::Ounce(value) => value.clone() * 6.0,
            Volume::Tablespoon(value) => value.clone() *3.0,
            Volume::Teaspoon(value) => value.clone() ,
            Volume::Liter(value) => value.clone() * 202.9,
            Volume::Milliliter(value) => value.clone() / 4.929,
            Volume::CubicMeter(value) => value.clone() * 202900.0,
            Volume::CubicFoot(value) => value.clone() * 5745.0,
            Volume::CubitInch(value) => value.clone() * 3.325,

        }
    }
    pub fn to_liter<'a>(self) -> f32 {
        match self {
            Volume::Gallon(value) => value.clone() / 3.785,
            Volume::Quart(value) => value.clone()* 1.057,
            Volume::Pint(value) => value.clone()/ 2.113,
            Volume::Cup(value) => value.clone() / 4.167,
            Volume::Ounce(value) => value.clone() / 33.814,
            Volume::Tablespoon(value) => value.clone() / 67.628,
            Volume::Teaspoon(value) => value.clone() / 202.9 ,
            Volume::Liter(value) => value.clone(),
            Volume::Milliliter(value) => value.clone() / 1000.0,
            Volume::CubicMeter(value) => value.clone() * 1000.0,
            Volume::CubicFoot(value) => value.clone() * 28.317,
            Volume::CubitInch(value) => value.clone() / 61.024,

        }
    }
    pub fn to_milliliter<'a>(self) -> f32 {
        match self {
            Volume::Gallon(value) => value.clone() * 3785.0,
            Volume::Quart(value) => value.clone() * 946.4,
            Volume::Pint(value) => value.clone() * 473.2,
            Volume::Cup(value) => value.clone() * 240.0,
            Volume::Ounce(value) => value.clone() * 29.574,
            Volume::Tablespoon(value) => value.clone() * 14.787,
            Volume::Teaspoon(value) => value.clone() * 4.929 ,
            Volume::Liter(value) => value.clone() * 1000.0,
            Volume::Milliliter(value) => value.clone(),
            Volume::CubicMeter(value) => value.clone() * 1e+6,
            Volume::CubicFoot(value) => value.clone() * 28320.0,
            Volume::CubitInch(value) => value.clone() * 16.387,

        }
    }
    pub fn to_cubic_meter<'a>(self) -> f32 {
        match self {
            Volume::Gallon(value) => value.clone() / 264.2,
            Volume::Quart(value) => value.clone() / 1057.0,
            Volume::Pint(value) => value.clone() / 2113.0,
            Volume::Cup(value) => value.clone() / 4167.0,
            Volume::Ounce(value) => value.clone() / 33810.0,
            Volume::Tablespoon(value) => value.clone() / 67630.0,
            Volume::Teaspoon(value) => value.clone() / 202900.0 ,
            Volume::Liter(value) => value.clone() / 1000.0,
            Volume::Milliliter(value) => value.clone() / 1e+6,
            Volume::CubicMeter(value) => value.clone(),
            Volume::CubicFoot(value) => value.clone() / 35.315,
            Volume::CubitInch(value) => value.clone() / 61020.0,

        }
    }
    pub fn to_cubic_foot<'a>(self) -> f32 {
        match self {
            Volume::Gallon(value) => value.clone() / 7.48,
            Volume::Quart(value) => value.clone() / 29.922,
            Volume::Pint(value) => value.clone() / 59.844,
            Volume::Cup(value) => value.clone() / 118.0,
            Volume::Ounce(value) => value.clone() / 957.5,
            Volume::Tablespoon(value) => value.clone() / 1915.0,
            Volume::Teaspoon(value) => value.clone() / 5745.0 ,
            Volume::Liter(value) => value.clone() / 28.317,
            Volume::Milliliter(value) => value.clone() / 28320.0,
            Volume::CubicMeter(value) => value.clone() * 35.315,
            Volume::CubicFoot(value) => value.clone() ,
            Volume::CubitInch(value) => value.clone() / 1728.0,

        }
    }
    pub fn to_cubic_inch<'a>(self) -> f32 {
        match self {
            Volume::Gallon(value) => value.clone() * 231.0,
            Volume::Quart(value) => value.clone() * 57.75,
            Volume::Pint(value) => value.clone() * 28.875,
            Volume::Cup(value) => value.clone() * 14.646,
            Volume::Ounce(value) => value.clone() * 1.805,
            Volume::Tablespoon(value) => value.clone() / 1.108,
            Volume::Teaspoon(value) => value.clone() / 3.325 ,
            Volume::Liter(value) => value.clone() * 61.024,
            Volume::Milliliter(value) => value.clone() / 16.387,
            Volume::CubicMeter(value) => value.clone() * 61020.0,
            Volume::CubicFoot(value) => value.clone() * 1728.0,
            Volume::CubitInch(value) => value.clone() ,

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
            "milliliter" => Ok(self.to_milliliter()),
            "cubic meter" => Ok(self.to_cubic_meter()),
            "cubic foot" => Ok(self.to_cubic_foot()),
            "cubic inch" => Ok(self.to_cubic_inch()),
            _ => Err("Invalid  unit of measurement."),
        }
    }
}




