pub mod response_models;
pub mod validation;

use std::ops::Deref;

use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use rocket::form::{self, FromFormField, ValueField};

// Permette di usare i tipi di chrono come parametri di query string e form data
// https://stackoverflow.com/questions/25413201/how-do-i-implement-a-trait-i-dont-own-for-a-type-i-dont-own
// https://github.com/SergioBenitez/Rocket/issues/602#issuecomment-380497269
// https://stackoverflow.com/a/65136340
pub struct NaiveDateForm(pub NaiveDate);
pub struct NaiveTimeForm(pub NaiveTime);
pub struct NaiveDateTimeForm(pub NaiveDateTime);

#[rocket::async_trait]
impl<'r> FromFormField<'r> for NaiveDateForm {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match NaiveDate::parse_from_str(&field.value, "%Y-%m-%d") {
            Ok(date) => Ok(NaiveDateForm(date)),
            Err(e) => Err(form::Error::validation(format!("Invalid date: {}", e)))?,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for NaiveTimeForm {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match NaiveTime::parse_from_str(&field.value, "%H:%M:%S%.3f") {
            Ok(time) => Ok(NaiveTimeForm(time)),
            Err(_) => match NaiveTime::parse_from_str(&field.value, "%H:%M") {
                Ok(time) => Ok(NaiveTimeForm(time)),
                Err(e) => Err(form::Error::validation(format!("Invalid time: {}", e)))?,
            },
        }
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for NaiveDateTimeForm {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match NaiveDateTime::parse_from_str(&field.value, "%Y-%m-%dT%H:%M:%S%.3f") {
            Ok(datetime) => Ok(NaiveDateTimeForm(datetime)),
            Err(_) => match NaiveDateTime::parse_from_str(&field.value, "%Y-%m-%dT%H:%M") {
                Ok(datetime) => Ok(NaiveDateTimeForm(datetime)),
                Err(e) => Err(form::Error::validation(format!("Invalid datetime: {}", e)))?,
            },
        }
    }
}

impl Deref for NaiveDateForm {
    type Target = NaiveDate;
    fn deref(&self) -> &NaiveDate {
        &self.0
    }
}

impl Deref for NaiveTimeForm {
    type Target = NaiveTime;
    fn deref(&self) -> &NaiveTime {
        &self.0
    }
}

impl Deref for NaiveDateTimeForm {
    type Target = NaiveDateTime;
    fn deref(&self) -> &NaiveDateTime {
        &self.0
    }
}
