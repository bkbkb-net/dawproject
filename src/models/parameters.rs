use thiserror::Error;

use crate::repositories::project::{BoolParameterType, IntegerParameterType, RealParameterType};

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum ParameterError {
    #[error("Value {0} is greater than max {1}")]
    SetMaxError(String, String),
    #[error("Value {0} is less than min {1}")]
    SetMinError(String, String),
    #[error("ParseFloatError: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}

impl IntegerParameterType {
    /// set value of parameter
    /// return true if value is changed
    /// return false if value is not changed
    /// return error if value is out of range
    pub fn set_value(&mut self, value: i32) -> Result<bool, ParameterError> {
        if let Some(max) = self.max {
            if value > max {
                return Err(ParameterError::SetMaxError(
                    value.to_string(),
                    max.to_string(),
                ));
            }
        }

        if let Some(min) = self.min {
            if value < min {
                return Err(ParameterError::SetMinError(
                    value.to_string(),
                    min.to_string(),
                ));
            }
        }

        if self.value != Some(value) {
            self.value = Some(value);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// get value of parameter
    pub fn unchecked_value(&self) -> i32 {
        self.value.unwrap()
    }
}

impl RealParameterType {
    pub fn get_max(&self) -> Result<Option<f64>, ParameterError> {
        self.max
            .as_ref()
            .map(|max| max.parse::<f64>().map_err(ParameterError::ParseFloatError))
            .transpose()
    }

    pub fn get_min(&self) -> Result<Option<f64>, ParameterError> {
        self.min
            .as_ref()
            .map(|min| min.parse::<f64>().map_err(ParameterError::ParseFloatError))
            .transpose()
    }

    pub fn get_value(&self) -> Result<Option<f64>, ParameterError> {
        self.value
            .as_ref()
            .map(|value| {
                value
                    .parse::<f64>()
                    .map_err(ParameterError::ParseFloatError)
            })
            .transpose()
    }

    /// set value of parameter
    /// return true if value is changed
    /// return false if value is not changed
    /// return error if value is out of range
    pub fn set_value(&mut self, value: f64) -> Result<bool, ParameterError> {
        if let Some(max) = self.get_max()? {
            if value > max {
                return Err(ParameterError::SetMaxError(
                    value.to_string(),
                    max.to_string(),
                ));
            }
        }
        if let Some(min) = self.get_min()? {
            if value < min {
                return Err(ParameterError::SetMinError(
                    value.to_string(),
                    min.to_string(),
                ));
            }
        }

        if self.get_value()? != Some(value) {
            self.value = Some(value.to_string());
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// get value of parameter
    pub fn unchecked_value(&self) -> f64 {
        self.get_value().unwrap().unwrap()
    }
}

impl BoolParameterType {
    pub fn unchecked_value(&self) -> bool {
        self.value.unwrap()
    }
}
