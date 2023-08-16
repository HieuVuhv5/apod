use argon2::Config;
use axum::extract::{Path, Query, State};
use axum::response::{Html, Response};
use axum::{Form, Json};
use http::header::{LOCATION, SET_COOKIE};
use http::{HeaderValue, StatusCode};
use hyper::Body;
use jsonwebtoken::Header;
use serde_json::{json, Value};
use tera::Context;
use tera::Tera;
use tracing::error;
use reqwest::blocking;

use reqwest::Error;
//use serde_json::Error;
use reqwest::blocking::Client;
use std::fmt::Display;


use crate::db::Store;
use crate::error::AppError;
use crate::get_timestamp_after_8_hours;
use crate::models::apod::{Apod, ApodId, GetApodById,  CreateApod, ApodByDay};
use crate::models::page::PagePackage;
use crate::models::user::{Claims, OptionalClaims, User, UserSignup, KEYS};
use crate::models::gallery::UserGallery;
use crate::template::TEMPLATES;

#[allow(dead_code)]
pub async fn root(
    State(mut am_database): State<Store>,
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    context.insert("name", &"hieu");

    let template_name = if let Some(claims_data) = claims {
        error!("Setting claims and is_logged_in is TRUE now");
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);
        // Get all the page data
      //  let page_packages = am_database.get_all_question_pages().await?;
      //  context.insert("page_packages", &page_packages);

        "index.html" // Use the new template when logged in
    } else {
        // Handle the case where the user isn't logged in
        error!("is_logged_in is FALSE now");
        context.insert("is_logged_in", &false);
        "index.html" // Use the original template when not logged in
    };

    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered))
}


#[allow(dead_code)]
pub async fn get_register(
    State(mut am_database): State<Store>,
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    context.insert("name", &"hieu");
    context.insert("is_logged_in", &false);

    let template_name = "register.html";


    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered))
}




#[allow(dead_code)]
pub async fn search_page(
    State(mut am_database): State<Store>,
    OptionalClaims(claims): OptionalClaims,
    Form(creds): Form<ApodByDay>,
) -> Result<Html<String>, AppError> {

    if creds.date.is_empty()  {
        return Err(AppError::MissingCredentials);
    }

    let mut context = Context::new();
    context.insert("name", &"hieu");

    let template_name = if let Some(claims_data) = claims {

        error!("on working on the search wit date {}",creds.date);
        error!("Setting claims for search page and is_logged_in is TRUE now");
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);

        // turn on the flag for display searching result data.
        context.insert("is_searching", &true);

        //now get apod- package for the searching date .
        let mut apod_db =  am_database.get_apod_by_date(creds.date.to_owned());

        let apod_src:Apod = match apod_db.await {
            Ok(t) => t,
            Err(er)=> {
                // if not exist in database, send request to Nasa api to get
                let apod_sr = get_apod_url(creds.date);
                let apod_url:CreateApod = match apod_sr.await {
                    Ok(t)=> t,
                    Err(er) => panic!("errre "),
                };
                // add new data to database
                 let apod_src = am_database
                    .add_apod(apod_url.title, apod_url.explanation, apod_url.date, apod_url.hdurl, apod_url.url)
                    .await?;
                apod_src


            }

        };

        let package = PagePackage {
                    apod: apod_src,
                    count: 1,
                    is_in_gallery: 0
                };

                // add to context of html
                    context.insert("package", &package);
        "index.html" // Use the new template when logged in
    } else {
        // Handle the case where the user isn't logged in
        error!("is_logged_in is FALSE now");
        context.insert("is_logged_in", &false);
        "index.html" // Use the original template when not logged in
    };

    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered))
}




#[allow(dead_code)]
pub async fn apod_pages(
    State(mut am_database): State<Store>,
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {

    let mut context = Context::new();
    context.insert("name", "Hieu Vu");
    // check user_login
    let template_name = if let Some(claims_data) = claims {
        error!("Setting claims and is_logged_in is TRUE now");
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);
        // Get all the page data
        let packages = am_database.get_apods_page().await?;
        context.insert("packages", &packages);
        let item_count = packages.len();
        context.insert("item_count",&item_count);
        "apods.html"
    } else {
        // Handle the case where the user isn't logged in
        error!("is_logged_in is FALSE now");
        context.insert("is_logged_in", &false);
        "index.html" // Use the original template when not logged in
    };
    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered))

}

/* handler for request ./gallery */
#[allow(dead_code)]

pub async fn gallery_page(
    State(mut am_database): State<Store>,
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    context.insert("name", "Hieu Vu");
    // check user_login
    let template_name = if let Some(claims_data) = claims {
        error!("Setting claims and is_logged_in is TRUE now");
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);
        // Get all the gallery page  data
        let packages = am_database.get_gallery_page(claims_data.email.to_owned()).await?;
        context.insert("packages", &packages);
        let item_count = packages.len();
        context.insert("item_count",&item_count);
        "gallery.html"
    } else {
        // Handle the case where the user isn't logged in
        error!("is_logged_in is FALSE now");
        context.insert("is_logged_in", &false);
        "index.html" // Use the original template when not logged in
    };
    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered))




}


#[allow(dead_code)]
pub async fn slideshow_page(
    State(mut am_database): State<Store>,
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {

    let mut context = Context::new();
    context.insert("name", "Hieu Vu");
    // check user_login
    let template_name = if let Some(claims_data) = claims {
        error!("Setting claims and is_logged_in is TRUE now");
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);
        // Get all the gallery page  data
        let packages = am_database.get_gallery_page(claims_data.email.to_owned()).await?;
        context.insert("packages", &packages);
        let item_count = packages.len();
        context.insert("item_count",&item_count);

        "slide.html"
    } else {
        // Handle the case where the user isn't logged in
        error!("is_logged_in is FALSE now");
        context.insert("is_logged_in", &false);
        "index.html" // Use the original template when not logged in
    };
    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered))

}

pub async fn register(
    State(mut database): State<Store>,
    Form(mut credentials): Form<UserSignup>,
) -> Result<Response<Body>, AppError> {
    // We should also check to validate other things at some point like email address being in right format

    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredentials);
    }

    if credentials.password != credentials.confirm_password {
        return Err(AppError::MissingCredentials);
    }

 error!("insert for user: {} and password{}",credentials.email,credentials.password );
    let mut  response_body;
    // Check to see if there is already a user in the database with the given email address
    //let existing_user = database.get_user(&credentials.email).await;
    // check if email already in database .
    let mut flag  = database.check_user(credentials.email.to_owned()).await?;

    // flag = 2;
    if flag > 0 {


  //  if let Ok(_) = existing_user {
        //return Err(AppError::UserAlreadyExists);
         response_body = r#"
        <script>
            alert('Error!!!  The email accout had been used, please try again!!!');
            window.location.href = '/';
        </script>
    "#;


    }
        else {

            // Here we're assured that our credentials are valid and the user doesn't already exist
            // hash their password
            let hash_config = Config::default();
            let salt = std::env::var("SALT").expect("Missing SALT");
            let hashed_password = match argon2::hash_encoded(
                credentials.password.as_bytes(),
                // If you'd like unique salts per-user, simply pass &[] and argon will generate them for you
                salt.as_bytes(),
                &hash_config,
            ) {
                Ok(result) => result,
                Err(_) => {
                    return Err(AppError::Any(anyhow::anyhow!("Password hashing failed")));
                }
            };

            credentials.password = hashed_password;

            let new_user = database.create_user(credentials).await?;

            response_body = r#"
        <script>
            alert('Your accout has been created!!!!');
            window.location.href = '/';
        </script>
        "#;
        }
    let response = Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(Body::from(response_body))
        .expect("Failed to build response");

    Ok(response)



   // Ok(new_user)
}

/* function add apod to user gallery */
pub async fn add_gallery(
    State(mut database): State<Store>,
    Form(creds): Form<UserGallery>,
) -> Result<Response<Body>, AppError> {

    if creds.email.is_empty() {

        return Err(AppError::MissingCredentials);

    }
    error!("Add to gallery with email {} and apodId {}",creds.email, creds.apod_id);

    // call get update to db //
    let u_gallery =database.add_user_gallery(creds.email, creds.apod_id).await?;

    error!("After run Add to gallery with email {} and apodId {}",u_gallery.email, u_gallery.apod_id);

    let mut response = Response::builder()
        .status(StatusCode::FOUND)
        .body(Body::empty())
        .unwrap();

    response
        .headers_mut()
        .insert(LOCATION, HeaderValue::from_static("/gallery"));


    Ok(response)
}

/* function add apod to user gallery from apods page*/
pub async fn add_gallery_from_apod(
    State(mut database): State<Store>,
    Form(creds): Form<UserGallery>,
) -> Result<Response<Body>, AppError> {

    if creds.email.is_empty() {

        return Err(AppError::MissingCredentials);

    }
    error!("Add to gallery with email {} and apodId {}",creds.email, creds.apod_id);

    // call get update to db //
    let u_gallery =database.add_user_gallery(creds.email, creds.apod_id).await?;

    error!("After run Add to gallery with email {} and apodId {}",u_gallery.email, u_gallery.apod_id);

    let mut response = Response::builder()
        .status(StatusCode::FOUND)
        .body(Body::empty())
        .unwrap();

    response
        .headers_mut()
        .insert(LOCATION, HeaderValue::from_static("/apod"));


    Ok(response)
}



/* Function delete gallery for request /delete_gallery -> delete from db and return gallery page. */

pub async fn delete_gallery(
    State(mut database): State<Store>,
    Form(creds): Form<UserGallery>,
) -> Result<Response<Body>, AppError> {

    if creds.email.is_empty() {

        return Err(AppError::MissingCredentials);

    }
    //error!("Add to gallery with email {} and apodId {}",creds.email, creds.apod_id);

    // call get update to db //
    database.delete_user_gallery(creds.email, creds.apod_id).await?;



    let mut response = Response::builder()
        .status(StatusCode::FOUND)
        .body(Body::empty())
        .unwrap();

    response
        .headers_mut()
        .insert(LOCATION, HeaderValue::from_static("/gallery"));


    Ok(response)
}





pub async fn login(
    State(mut database): State<Store>,
    Form(creds): Form<User>,
) -> Result<Response<Body>, AppError> {
    if creds.email.is_empty() || creds.password.is_empty() {
        return Err(AppError::MissingCredentials);
    }

    let mut is_password_correct:bool= false;
    // check if email already in database .
    let mut flag  = database.check_user(creds.email.to_owned()).await?;

   // flag = 2;
    if flag > 0 {
        let existing_user: User = database.get_user(&creds.email).await?;

         is_password_correct =
            match argon2::verify_encoded(&*existing_user.password, creds.password.as_bytes()) {
                Ok(result) => result,
                Err(_) => {
                    // return Err(AppError::InternalServerError);
                    false
                }
            };
        }

    if !is_password_correct {
       // return Err(AppError::InvalidPassword);

        let response_body = r#"
        <script>
            alert('Incorrect Username/Password, please try again!!!');
            window.location.href = '/';
        </script>
    "#;

        let response = Response::builder()
            .header("Content-Type", "text/html; charset=utf-8")
            .body(Body::from(response_body))
            .expect("Failed to build response");

        Ok(response)


    }
    else {

        // at this point we've authenticated the user's identity
        // create JWT to return
        let claims = Claims {
            id: 0,
            email: creds.email.to_owned(),
            exp: get_timestamp_after_8_hours(),
        };

        let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding)
            .map_err(|_| AppError::MissingCredentials)?;

        let cookie = cookie::Cookie::build("jwt", token).http_only(true).finish();

        let mut response = Response::builder()
            .status(StatusCode::FOUND)
            .body(Body::empty())
            .unwrap();

        response
            .headers_mut()
            .insert(LOCATION, HeaderValue::from_static("/"));
        response.headers_mut().insert(
            SET_COOKIE,
            HeaderValue::from_str(&cookie.to_string()).unwrap(),
        );

        Ok(response)
    }
}

pub async fn protected(claims: Claims) -> Result<String, AppError> {
    Ok(format!(
        "Welcome to the PROTECTED area :) \n Your claim data is: {}",
        claims
    ))
}


pub async fn logout(claims: Claims) ->Result<Response<Body>, AppError> {


    let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AppError::MissingCredentials)?;

    let mut cookie = cookie::Cookie::build("jwt", token).http_only(true).finish();
    cookie.make_removal();
    let mut response = Response::builder()
        .status(StatusCode::FOUND)
        .body(Body::empty())
        .unwrap();

    response
        .headers_mut()
        .insert(LOCATION, HeaderValue::from_static("/"));
    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );

    Ok(response)

}



pub async fn create_apod(
    State (mut am_database): State<Store>,
    Json(apod): Json<CreateApod>,
)->Result<Json<Apod>, AppError>{
    let apod = am_database
        .add_apod(apod.title, apod.explanation, apod.date, apod.hdurl, apod.url)
        .await?;
    Ok(Json(apod))
}

pub async fn get_apod_url(data: String)-> Result<CreateApod , Error>{
    //let response = reqwest::blocking::get("https://api.nasa.gov/planetary/apod?api_key=GjmkqxDe1nZLLdmixSZJKcFlrGGa6MP08aBDSCKC & date =2023-07-07").unwrap();

   // let response = reqwest::blocking::get("https://api.nasa.gov/planetary/apod?api_key=DEMO_KEY").unwrap();

    //let api_key=std::env::var("API_KEY");
        let api_key = "GjmkqxDe1nZLLdmixSZJKcFlrGGa6MP08aBDSCKC";

    let request = format!("https://api.nasa.gov/planetary/apod?api_key={api_key}&date={data}");
    let apod = reqwest::get(request).await?.json::<CreateApod>().await?;



    Ok(apod)
}
/*
fn get_apod_from_nasa()->Result< Apod,Error>{

    let response  = get_apod_url("10".to_string());
    let text = match response {
        Ok(t) => t,
        Err(er) => panic!("Problem {:?}", er),
    };

    let v = serde_json::from_str(&text);

    let apodid =ApodId(0);

    let nasa_apod =Apod { id: apodid, title:"title".to_string(), explanation:"explanation".to_string(), date: "date".to_string(), hdurl: "hdurl".to_string(), url: "url".to_string()};

    Ok(nasa_apod)

}
*/
pub async fn get_apod_by_date(
    State(mut am_database): State<Store>,
    Path(query): Path<String>,
)->Result<Json<Apod>,AppError>{

    // search from database first .
     let mut apod_db =  am_database.get_apod_by_date(query.to_owned());

        let apod:Apod = match apod_db.await {
         Ok(t) => t,
         Err(er)=> {
             // if not exist in database, send request to Nasa api to get
             let apod_sr = get_apod_url(query);
             let apod_url:CreateApod = match apod_sr.await {
                 Ok(t)=> t,
                 Err(er) => panic!("errre "),
             };
             // add new data to database
              let apod = am_database
                 .add_apod(apod_url.title, apod_url.explanation, apod_url.date, apod_url.hdurl, apod_url.url)
                 .await?;
                apod


         }

     };


    Ok(Json(apod))


}

pub async fn get_all_apod(
    State(mut am_database): State<Store>,
) -> Result<Json<Vec<Apod>>, AppError>{
    let all_apods = am_database.get_all_apod().await?;
    Ok(Json(all_apods))
}


/* handle error with error message .
 */
