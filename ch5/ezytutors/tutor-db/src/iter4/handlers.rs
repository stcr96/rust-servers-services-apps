use super::db_access::*;
use super::models::Course;
use super::state::AppState;
use std::convert::TryFrom;
use super::errors::EzyTutorError;

use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
  let health_check_response = &app_state.health_check_response;
  let mut visit_count = app_state.visit_count.lock().unwrap();
  let response = format!("{} {} times", health_check_response, visit_count);
  *visit_count += 1;
  HttpResponse::Ok().json(&response)
}

pub async fn get_courses_for_tutor(
  app_state: web::Data<AppState>,
  params: web::Path<(i32,)>,
) -> Result<HttpResponse, EzyTutorError> {
  let tuple = params.into_inner();
  let tutor_id: i32 = i32::try_from(tuple.0).unwrap();
  get_courses_for_tutor_db(&app_state.db, tutor_id)
    .await
    .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_details(
  app_state: web::Data<AppState>,
  web::Path((tutor_id, course_id)): web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorError> {
  get_course_details_db(&app_state.db, tutor_id, course_id)
    .await
    .map(|course| HttpResponse::Ok().json(course))
  HttpResponse::Ok().json(course)
}

pub async fn post_new_course(
  new_course: web::Json<Course>,
  app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzyTutorError> {
  post_new_course_db(&app_state.db, new_course.into())
    .await
    .map(|course| HttpResponse::Ok().json(course))
}

#[actix_rt::test]
async fn get_all_courses_success() {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
  let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
  let app_state: web::Data<AppState> = web::Data::new(AppState {
    health_check_response: "".to_string(),
    visit_count: Mutex::new(0),
    db: pool,
  });
  let tutor_id: web::Path<i32> = web::Path::from(1);
  let resp = get_courses_for_tutor(app_state, tutor_id).await.unwrap();
  assert_eq!(resp.status(), StatusCode::OK);
}