use std::borrow::Cow;

use chrono::Local;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use regex::Regex;
use validator::ValidationError;

pub fn is_past_date(date: &&NaiveDate) -> Result<(), ValidationError> {
    let today = Local::now().date_naive();
    if **date < today {
        Ok(())
    } else {
        Err(ValidationError::new("not_past_date")
            .with_message(Cow::Borrowed("The date must be in the past")))
    }
}

pub fn is_valid_phone(phone: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"^(\+\d{1,2}\s?)?\(?\d{3}\)?[\s.-]?\d{3}[\s.-]?\d{4}$").unwrap();
    if re.is_match(phone) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_phone")
            .with_message(Cow::Borrowed("The phone number is not valid")))
    }
}

pub fn is_past_datetime(datetime: &NaiveDateTime) -> Result<(), ValidationError> {
    let now = Local::now().naive_local();
    if *datetime < now {
        Ok(())
    } else {
        Err(ValidationError::new("not_past_datetime")
            .with_message(Cow::Borrowed("The datetime must be in the past")))
    }
}

pub fn is_future_datetime(datetime: &NaiveDateTime) -> Result<(), ValidationError> {
    let now = Local::now().naive_local();
    if *datetime > now {
        Ok(())
    } else {
        Err(ValidationError::new("not_future_datetime")
            .with_message(Cow::Borrowed("The datetime must be in the future")))
    }
}
