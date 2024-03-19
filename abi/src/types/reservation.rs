use crate::{convert_to_timestamp, convert_to_utc_time, Error, Reservation, ReservationStatus};
use chrono::{DateTime, FixedOffset, Utc};
use std::ops::Range;

impl Reservation {
    pub fn new_pending<T>(
        user_id: T,
        resource_id: T,
        start: DateTime<FixedOffset>,
        end: DateTime<FixedOffset>,
        note: T,
    ) -> Self
    where
        T: Into<String>,
    {
        Self {
            id: "".into(),
            user_id: user_id.into(),
            resource_id: resource_id.into(),
            start: Some(convert_to_timestamp(start.with_timezone(&Utc))),
            end: Some(convert_to_timestamp(end.with_timezone(&Utc))),
            note: note.into(),
            status: ReservationStatus::Pending as i32,
        }
    }

    pub fn validate(&self) -> Result<(), Error> {
        if self.user_id.is_empty() {
            return Err(Error::InvalidUserId(self.user_id.clone()));
        }
        if self.resource_id.is_empty() {
            return Err(Error::InvalidResourceId(self.resource_id.clone()));
        }
        if self.start.is_none() || self.end.is_none() {
            return Err(Error::InvalidTime);
        }
        let start = convert_to_utc_time(self.start.as_ref().unwrap().clone());
        let end = convert_to_utc_time(self.end.as_ref().unwrap().clone());
        if start >= end {
            return Err(Error::InvalidTime);
        }
        Ok(())
    }

    pub fn get_timespan(&self) -> Range<DateTime<Utc>> {
        let start = convert_to_utc_time(self.start.as_ref().unwrap().clone());
        let end = convert_to_utc_time(self.end.as_ref().unwrap().clone());
        Range { start, end }
    }
}
