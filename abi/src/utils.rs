use chrono::{DateTime, Utc};
use prost_types::Timestamp;

pub fn convert_to_utc_time(ts: Timestamp) -> DateTime<Utc> {
    DateTime::from_timestamp(ts.seconds, ts.nanos.try_into().unwrap()).unwrap()
}

pub fn convert_to_timestamp(dt: DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos().try_into().unwrap(),
    }
}
