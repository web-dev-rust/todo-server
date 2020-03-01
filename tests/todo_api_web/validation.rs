use todo_server::todo_api::core::{validate_jwt_info};
use todo_server::todo_api::model::auth::User;
use todo_server::todo_api_web::model::http::Clients;
use actix_web::http::StatusCode;

#[actix_rt::test]
async fn all_args_are_equal_is_accepted() {
    use dotenv::dotenv;
    dotenv().ok();

    let exec = Clients::new();
    let state = actix_web::web::Data::new(exec);

    let user = User::from("my@email.com".to_string(), "pass".to_string());
    let email = "my@email.com".to_string();

    let resp = validate_jwt_info(email.clone(), email, Ok(user), state).await;
    assert_eq!(resp.status(), StatusCode::ACCEPTED);
}

#[actix_rt::test]
async fn all_args_are_not_equal_is_unauth() {
    use dotenv::dotenv;
    dotenv().ok();

    let exec = Clients::new();
    let state = actix_web::web::Data::new(exec);

    let user = User::from("not_my@email.com".to_string(), "pass".to_string());
    let email = "my@email.com".to_string();

    let resp = validate_jwt_info(email.clone(), email, Ok(user), state).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}