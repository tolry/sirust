use std::collections::HashMap;
use std::ops::{Add, Sub, Mul};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum StandardUnit {
    Ampere,
    Meter,
    Kilogram,
    Second,
    Kelvin,
    Candela,
    Mole,
}

#[derive(Debug, Clone)]
pub struct Input {
    pub compound_unit: HashMap<StandardUnit, i16>,
    pub amount: f64,
}

impl Add for Input {
    type Output = Result<Input, String>;

    fn add(self, other: Input) -> Result<Input, String> {
        match self.compound_unit == other.compound_unit {
            true => Ok(Input {compound_unit: self.compound_unit, amount: self.amount + other.amount}),
            false => Err(format!("you fool {:?} vs {:?}", self.compound_unit, other.compound_unit))
        }
    }
}

impl Sub for Input {
    type Output = Result<Input, String>;

    fn sub(self, other: Input) -> Result<Input, String> {
        match self.compound_unit == other.compound_unit {
            true => Ok(Input {compound_unit: self.compound_unit, amount: self.amount - other.amount}),
            false => Err(format!("you fool {:?} vs {:?}", self.compound_unit, other.compound_unit))
        }
    }
}

impl Mul for Input {
    type Output = Result<Input, String>;

    fn mul(self, other: Input) -> Result<Input, String> {
        if self.compound_unit != other.compound_unit {
            return Err(format!("you fool {:?} vs {:?}", self.compound_unit, other.compound_unit))
        }

        let mut new_compound_unit = HashMap::new();
        for (unit, exponent) in &self.compound_unit {
            let mut new_exponent = *exponent;
            match other.compound_unit.get(unit) {
                Some(other_exponent) => new_exponent += other_exponent,
                None => {}
            }
            new_compound_unit.insert(unit.clone(), new_exponent);
        }

        return Ok(Input {compound_unit: new_compound_unit, amount: self.amount * other.amount});
    }
}

