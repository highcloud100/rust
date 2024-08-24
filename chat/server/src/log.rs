use lamport::Lamport;
use chrono::{DateTime, Utc};
use chrono_tz::{Asia::Seoul, Tz};

#[derive(Debug, Clone)]
pub struct Log{
    id : String, 
    lamport_clock : Lamport, 
    msg: String,
    real_clock : DateTime<Tz>,
}