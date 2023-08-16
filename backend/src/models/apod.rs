use crate::make_db_id;
use serde_derive::{Deserialize, Serialize};
use chrono::NaiveDate;

// This uses the `derive_more` crate to reduce the Display boilerplate (see below)
#[derive(Clone, Debug, Display, Serialize, Deserialize, sqlx::FromRow)]
#[display(
fmt = "id: {}, title: {}, explanation: {}, date: {}, hdurl: {}, url: {}",
id,
title,
explanation,
date,
hdurl,
url
)]
pub struct Apod {
    pub id: ApodId,
    pub title: String,
    pub explanation: String,
    pub date: String,
    pub hdurl: String,
    pub url: String,
}

impl Apod {
    #[allow(dead_code)]
    pub fn new(id: ApodId, title: String, explanation: String, date: String, hdurl: String, url: String) -> Self {
        Apod {
            id,
            title,
            explanation,
            date,
            hdurl,
            url,
        }
    }
}

make_db_id!(ApodId);

// Clients use this to create new requests
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApod {
    //pub copyright: String,
    pub date: String,
    pub explanation: String,
    pub hdurl: String,
   // pub service_version: String,
    //pub media_type: String,
    pub title: String,
    pub url: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApodByDay{
    pub date: String,
}

#[derive(Deserialize)]
pub struct GetApodById {
    pub apod_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateApod {
    pub id: ApodId,
    pub title: String,
    pub explanation: String,
    pub date: String,
    pub hdurl: String,
    pub url: String,
}
