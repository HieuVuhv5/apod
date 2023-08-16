use axum::Json;
use serde_json::Value;
use std::sync::{Arc, Mutex, RwLock};

use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use tracing::info;

use crate::models::page::PagePackage;
use crate::error::AppError;
use crate::models::apod::{Apod, ApodId, GetApodById, CreateApod};
use crate::models::gallery::UserGallery;
//use crate::models::answer::{Answer, AnswerId};
//use crate::models::comment::{Comment, CommentId, CommentReference};
//use crate::models::page::{AnswerWithComments, PagePackage, QuestionWithComments};
//use crate::models::question::{
 //   GetQuestionById, IntoQuestionId, Question, QuestionId, UpdateQuestion,
//};
use crate::models::user::{User, UserSignup};
use chrono::NaiveDate;
#[derive(Clone)]
pub struct Store {
    pub conn_pool: PgPool,

    //pub answers: Arc<RwLock<Vec<Answer>>>,
}

pub async fn new_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap()
}

impl Store {
    pub fn with_pool(pool: PgPool) -> Self {
        Self {
            conn_pool: pool,

           // answers: Default::default(),
        }
    }

    pub async fn test_database(&self) -> Result<(), sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&self.conn_pool)
            .await?;

        info!("{}", &row.0);

        assert_eq!(row.0, 150);
        Ok(())
    }

    /* check user email exist on system or not */

    pub async fn check_user(&self, email: String) -> Result<u8, AppError> {
        let row = sqlx::query!(
            r#"
                select * from users where email =$1
            "#,
            email,

        )
            .fetch_all(&self.conn_pool)
            .await?;

        if row.len()> 0
        {
             Ok(2)
        }
        else {
            Ok(0)
        }

    }


    pub async fn get_user(&self, email: &str) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
                SELECT email, password FROM users WHERE email = $1
            "#,
        )
            .bind(email)
            .fetch_one(&self.conn_pool)
            .await?;

        Ok(user)
    }

    pub async fn create_user(&self, user: UserSignup) -> Result<Json<Value>, AppError> {
        // TODO: Encrypt/bcrypt user passwords
        let result = sqlx::query("INSERT INTO users(email, password) values ($1, $2)")
            .bind(&user.email)
            .bind(&user.password)
            .execute(&self.conn_pool)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        if result.rows_affected() < 1 {
            Err(AppError::InternalServerError)
        } else {
            Ok(Json(
                serde_json::json!({"message": "User created successfully!"}),
            ))
        }
    }


    pub async fn get_all_apod(&mut self) -> Result<Vec<Apod>, AppError>{
        let rows =sqlx::query!(
            r#"
            Select * From apod
            "#
        )
            .fetch_all(&self.conn_pool)
            .await?;

        let apods: Vec<_> = rows
            .into_iter()
            .map(|row| {
                Apod {
                    id: row.id.into(),
                    title: row.title,
                    explanation: row.explanation,
                    date: row.date,
                    hdurl: row.hdurl,
                    url: row.url,
                }
            })
            .collect();
        Ok(apods)
    }
/*
    pub  async fn get_search_page(&self, date_str: String) -> Result<PagePackage, AppError>{



    }
    */

    pub  async fn get_apods_page(&self) -> Result<Vec<PagePackage>, AppError>
    {

       // let apods = self.get_all_apod().await?;

        let rows =sqlx::query("Select * from apod ORDER BY date DESC LIMIT 20")
            .fetch_all(&self.conn_pool)
            .await?;

        let mut count_grid = 1;
        let mut res = Vec::new();

        for row in rows {
            let apod_p =Apod{
                id: ApodId(row.get("id")),
                title: row.get("title"),
                explanation: row.get("explanation"),
                date : row.get("date"),
                hdurl: row.get("hdurl"),
                url: row.get("url"),
            };

            let package = PagePackage{
                apod: apod_p,
                count: count_grid,
                is_in_gallery: 0,
            };
            res.push(package);
            count_grid +=1;
        }
    //panic!("number of apod {}", res.len());
        Ok(res)
    }


    /* get user gallery page */

    pub  async fn get_gallery_page(&self, email_str: String) -> Result<Vec<PagePackage>, AppError>
    {
        let rows =sqlx::query("select * from apod inner join users_gallery ug on apod.id = ug.apod_id
where ug.email =$1")
            .bind(email_str)
            .fetch_all(&self.conn_pool)
            .await?;

        let mut count_grid = 1;
        let mut res = Vec::new();

        for row in rows {
            let apod_p =Apod{
                id: ApodId(row.get("id")),
                title: row.get("title"),
                explanation: row.get("explanation"),
                date : row.get("date"),
                hdurl: row.get("hdurl"),
                url: row.get("url"),
            };

            let package = PagePackage{
                apod: apod_p,
                count: count_grid,
                is_in_gallery: 1,
            };
            res.push(package);
            count_grid +=1;
        }
        //panic!("number of apod {}", res.len());
        Ok(res)
    }

/* Done user gallery */


    pub async fn add_apod(
        &mut self,
        title: String,
        explanation: String,
        date: String,
        hdurl: String,
        url: String,
    )->Result<Apod,AppError>{
        let res =sqlx::query!(
            r#"
            Insert into "apod"(title,explanation, date, hdurl, url)
            Values($1,$2,$3,$4,$5)
            Returning *
            "#,
            title,
            explanation,
            date,
            hdurl,
            url
        )
            .fetch_one(&self.conn_pool)
            .await?;
        let new_apod = Apod{
            id: ApodId(res.id),
            title: res.title,
            explanation: res.explanation,
            date: res.date,
            hdurl: res.hdurl,
            url: res.url,
        };
        Ok(new_apod)
    }

    pub async fn get_apod_by_date(
        &mut self,
        date: String,
    )->Result<Apod, AppError>{
        let row = sqlx::query!(
            r#"
            SELECT * FROM apod WHERE date =$1
            "#,
            date,
        )
        .fetch_one(&self.conn_pool)
            .await?;
        let apod = Apod {
            id: row.id.into(),
            title: row.title,
            explanation: row.explanation,
            date: row.date,
            hdurl: row.hdurl,
            url: row.url,
            };
        Ok(apod)
        }

    /* Function add apod to user's gallery */

    pub async fn add_user_gallery(
        &mut self,
        emailstr: String,
        apod_id: i32,
    )->Result<UserGallery, AppError>{


        /* solve with duplicate data
            run querry check record in databse first .
            if yes, do no thing.
            if no, insert to db.
         */
        let rows = sqlx::query! (
            r#"
            Select * from users_gallery where email =$1 and apod_id =$2
            "#,
            emailstr,
            apod_id
        )
            .fetch_all(&self.conn_pool)
            .await?;

        if rows.len() > 0 {
            // at least one mean data already add
            let user_gall = UserGallery{
                email: emailstr,
                apod_id: apod_id,
            };
            Ok(user_gall)
        }
        else {



        let res =sqlx::query!(
            r#"
            INSERT INTO users_gallery (email, apod_id)
            VALUES ($1, $2)
            RETURNING *
            "#,
            emailstr,
            apod_id,
        )
           .fetch_one(&self.conn_pool)
            .await?;



            let user_gall = UserGallery{
                email: res.email,
                apod_id: res.apod_id,
            };
            Ok(user_gall)
        }



        }
    /* Function delete APOD from user gallery */

    pub async fn delete_user_gallery(
        &mut self,
        emailstr: String,
        apod_id: i32,
    )->Result<(), AppError>{

        sqlx::query!(
            r#"
            delete from users_gallery where email =$1 and apod_id = $2
            "#,
            emailstr,
            apod_id
        )
            .execute(&self.conn_pool)
            .await
            .unwrap();

        Ok(())


    }


/*
    pub async fn get_user_gallery(
        &mut self,
        emailstr: String,
    )->Result<UserGallery, AppError>{
        let rows_gall = sqlx::query("SELECT * FROM users_gallery WHERE email = $1")
            .bind(emailstr.to_owned())
            .fetch_all(&self.conn_pool)
            .await?;



        let mut vec_r:Vec<i32> = Vec::new();

        for row in rows_gall {
            vec_r.push(row.get("apod_id"));

        };

        let user_gall = UserGallery{
            user_email: emailstr,
            vec_apods: Some(vec_r),
        };

        Ok(user_gall)

    }

*/

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
