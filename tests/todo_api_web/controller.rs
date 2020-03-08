#[cfg(test)]
mod ping_readiness {
    use todo_server::todo_api_web::model::http::Clients;
    use todo_server::todo_api_web::routes::app_routes;

    use actix_service::Service;
    use actix_web::{http::StatusCode, test, App};
    use bytes::Bytes;
    use dotenv::dotenv;

    #[actix_rt::test]
    async fn test_ping_pong() {
        dotenv().ok();
        let mut app =
            test::init_service(App::new().data(Clients::new()).configure(app_routes)).await;

        let req = test::TestRequest::get().uri("/ping").to_request();
        let resp = test::read_response(&mut app, req).await;

        assert_eq!(resp, Bytes::from_static(b"pong"));
    }

    #[actix_rt::test]
    async fn test_readiness_ok() {
        dotenv().ok();
        let mut app =
            test::init_service(App::new().data(Clients::new()).configure(app_routes)).await;

        let req = test::TestRequest::with_uri("/~/ready").to_request();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::ACCEPTED);
    }

    #[actix_rt::test]
    async fn not_found_route() {
        dotenv().ok();
        let mut app =
            test::init_service(App::new().data(Clients::new()).configure(app_routes)).await;

        let req = test::TestRequest::get().uri("/crazy-route").to_request();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}

#[cfg(test)]
mod create_todo {
    use actix_web::{test, App};
    use dotenv::dotenv;
    use serde_json::from_str;
    use todo_server::todo_api_web::model::http::Clients;
    use todo_server::todo_api_web::{model::todo::TodoIdResponse, routes::app_routes};

    use crate::helpers::read_json;

    #[actix_rt::test]
    async fn valid_todo_post() {
        dotenv().ok();
        let mut app =
            test::init_service(App::new().data(Clients::new()).configure(app_routes)).await;

        let req = test::TestRequest::post()
            .uri("/api/create")
            .header("Content-Type", "application/json")
            .set_payload(read_json("post_todo.json").as_bytes().to_owned())
            .to_request();

        let resp = test::read_response(&mut app, req).await;

        let id: TodoIdResponse = from_str(&String::from_utf8(resp.to_vec()).unwrap()).unwrap();
        assert!(uuid::Uuid::parse_str(&id.get_id()).is_ok());
    }
}

#[cfg(test)]
mod read_all_todos {
    use actix_service::Service;
    use actix_web::{http::StatusCode, test, App};
    use dotenv::dotenv;
    use serde_json::from_str;
    use todo_server::todo_api_web::model::http::Clients;
    use todo_server::todo_api_web::{model::todo::TodoCardsResponse, routes::app_routes};

    use crate::helpers::{mock_get_todos, read_json};

    #[actix_rt::test]
    async fn test_todo_index_ok() {
        dotenv().ok();
        let mut app =
            test::init_service(App::new().data(Clients::new()).configure(app_routes)).await;

        let req = test::TestRequest::with_uri("/api/index").to_request();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_todo_cards_count() {
        dotenv().ok();
        let mut app =
            test::init_service(App::new().data(Clients::new()).configure(app_routes)).await;

        let post_req = test::TestRequest::post()
            .uri("/api/create")
            .header("Content-Type", "application/json")
            .set_payload(read_json("post_todo.json").as_bytes().to_owned())
            .to_request();

        let _ = app.call(post_req).await.unwrap();
        let req = test::TestRequest::with_uri("/api/index").to_request();
        let resp = test::read_response(&mut app, req).await;

        let todo_cards: TodoCardsResponse =
            from_str(&String::from_utf8(resp.to_vec()).unwrap()).unwrap();
        assert_eq!(todo_cards.cards.len(), 1);
    }

    #[actix_rt::test]
    async fn test_todo_cards_with_value() {
        dotenv().ok();
        let mut app =
            test::init_service(App::new().data(Clients::new()).configure(app_routes)).await;

        let post_req = test::TestRequest::post()
            .uri("/api/create")
            .header("Content-Type", "application/json")
            .set_payload(read_json("post_todo.json").as_bytes().to_owned())
            .to_request();

        let _ = app.call(post_req).await.unwrap();
        let req = test::TestRequest::with_uri("/api/index").to_request();
        let resp = test::read_response(&mut app, req).await;

        let todo_cards: TodoCardsResponse =
            from_str(&String::from_utf8(resp.to_vec()).unwrap()).unwrap();
        assert_eq!(todo_cards.cards, mock_get_todos());
    }
}

#[cfg(test)]
mod auth {
    use crate::helpers::read_json;
    use actix_web::{http::StatusCode, test, App};
    use dotenv::dotenv;
    use todo_server::todo_api_web::model::http::Clients;
    use todo_server::todo_api_web::routes::app_routes;

    #[actix_rt::test]
    async fn signup_returns_created_status() {
        dotenv().ok();
        let mut app =
            test::init_service(App::new().data(Clients::new()).configure(app_routes)).await;

        let signup_req = test::TestRequest::post()
            .uri("/auth/signup")
            .header("Content-Type", "application/json")
            .set_payload(read_json("signup.json").as_bytes().to_owned())
            .to_request();

        let resp = test::call_service(&mut app, signup_req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_rt::test]
    async fn login_returns_token() {
        dotenv().ok();
        let mut app =
            test::init_service(App::new().data(Clients::new()).configure(app_routes)).await;

        let login_req = test::TestRequest::post()
            .uri("/auth/login")
            .header("Content-Type", "application/json")
            .set_payload(read_json("signup.json").as_bytes().to_owned())
            .to_request();

        let resp_body = test::read_response(&mut app, login_req).await;

        let jwt: String = String::from_utf8(resp_body.to_vec()).unwrap();

        assert!(jwt.contains("token"));
    }

    #[actix_rt::test]
    async fn logout_accepted() {
        dotenv().ok();
        let mut app =
            test::init_service(App::new().data(Clients::new()).configure(app_routes)).await;

        let logout_req = test::TestRequest::delete()
            .uri("/auth/logout")
            .header("Content-Type", "application/json")
            .header("x-auth", "token")
            .set_payload(read_json("logout.json").as_bytes().to_owned())
            .to_request();

        let resp = test::call_service(&mut app, logout_req).await;
        assert_eq!(resp.status(), StatusCode::ACCEPTED);
    }
}

#[cfg(test)]
mod middleware {
    use actix_web::{test, App, http::StatusCode};
    use dotenv::dotenv;
    use todo_server::todo_api_web::model::http::Clients;
    use todo_server::todo_api_web::routes::app_routes;

    use crate::helpers::read_json;

    #[actix_rt::test]
    async fn bad_request_todo_post() {
        dotenv().ok();
        let mut app =
            test::init_service(
                App::new()
                .data(Clients::new())
                .wrap(todo_server::todo_api_web::middleware::Authentication)
                .configure(app_routes)
            ).await;

        let req = test::TestRequest::post()
            .uri("/api/create")
            .header("Content-Type", "application/json")
            .set_payload(read_json("post_todo.json").as_bytes().to_owned())
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn good_token_todo_post() {
        dotenv().ok();
        let mut app =
            test::init_service(
                App::new()
                .data(Clients::new())
                .wrap(todo_server::todo_api_web::middleware::Authentication)
                .configure(app_routes)
            ).await;

        let req = test::TestRequest::post()
            .uri("/api/create")
            .header("Content-Type", "application/json")
            .header("x-auth", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6Ijc1NjJiZjUzLTYxNTYtNDMzYi1hMjAxLTkwYmJjNzRiMDEyNyIsImVtYWlsIjoibXlAZW1haWwuY29tIiwiZXhwaXJlc19hdCI6IjMwMjAtMTEtMjhUMTI6MDA6MDkifQ.hom6KvmmLIuu3dLCSUrOK9KBWyUb0fvdX4hIay52UIY")
            .set_payload(read_json("post_todo.json").as_bytes().to_owned())
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        println!("{:?}", resp);
        assert_eq!(resp.status(), StatusCode::CREATED);
    }
}

#[cfg(test)]
mod show_by_id {
    use actix_web::{test, App};
    use dotenv::dotenv;
    use todo_server::todo_api_web::model::{
        http::Clients,
        todo::TodoCard,
    };
    use todo_server::todo_api_web::routes::app_routes;
    use serde_json::from_str;
    use crate::helpers::{mock_get_todos};

    #[actix_rt::test]
    async fn test_todo_card_by_id() {
        dotenv().ok();
        let mut app =
            test::init_service(App::new().data(Clients::new()).configure(app_routes)).await;

        let req = test::TestRequest::with_uri("/api/show/544e3675-19f5-4455-9ed9-9ccc577f70fe").to_request();
        let resp = test::read_response(&mut app, req).await;

        let todo_card: TodoCard =
            from_str(&String::from_utf8(resp.to_vec()).unwrap()).unwrap();
        assert_eq!(&todo_card, mock_get_todos().get(0usize).unwrap());
    }

    #[actix_rt::test]
    async fn test_todo_card_without_uuid() {
        dotenv().ok();
        let mut app =
            test::init_service(App::new().data(Clients::new()).configure(app_routes)).await;

        let req = test::TestRequest::with_uri("/api/show/fake-uuid").to_request();
        let resp = test::read_response(&mut app, req).await;

        let message = String::from_utf8(resp.to_vec()).unwrap();
        assert_eq!(&message, "Id must be a Uuid::V4");
    }
}