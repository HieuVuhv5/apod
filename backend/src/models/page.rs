
use crate::models::apod::Apod;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PagePackage {
    pub apod: Apod,
   // pub answers: Vec<AnswerWithComments>,
    pub count : i32,
    // this is flag to mark if this apod was in the user gallery,
    // this help to on/ of add to gallery function.
    pub is_in_gallery: i8,
}

impl IntoResponse for PagePackage {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
/*
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuestionWithComments {
    pub question: Question,
    pub comments: Vec<Comment>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnswerWithComments {
    pub answer: Answer,
    pub comments: Vec<Comment>,
}
*/