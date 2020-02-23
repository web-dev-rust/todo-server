mod ping_readiness {
    use todo_server::todo_api_web::routes::app_routes;
    use todo_server::todo_api_web::model::http::Clients;

    use bytes::Bytes;
    use actix_web::{
        test, App,
        http::StatusCode,
    };
    use actix_service::Service;
    use dotenv::dotenv;

    #[actix_rt::test]
    async fn test_ping_pong() {
        dotenv().ok();
        let mut app = test::init_service(
            App::new()
                .data(Clients::new())
                .configure(app_routes)
        ).await;

        let req = test::TestRequest::get()
            .uri("/ping")
            .to_request();
        let resp = test::read_response(&mut app, req).await;

        assert_eq!(resp, Bytes::from_static(b"pong"));
    }

    #[actix_rt::test]
    async fn test_readiness_ok() {
        dotenv().ok();
        let mut app = test::init_service(
            App::new()
                .data(Clients::new())
                .configure(app_routes)
        ).await;
    
        let req = test::TestRequest::with_uri("/~/ready").to_request();
    
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::ACCEPTED);
    }

    #[actix_rt::test]
    async fn not_found_route() {
        dotenv().ok();
        let mut app = test::init_service(
            App::new()
                .data(Clients::new())
                .configure(app_routes)
        ).await;

        let req = test::TestRequest::get()
            .uri("/crazy-route")
            .to_request();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}

#[cfg(test)]
mod create_todo {
    use todo_server::todo_api_web::{
        model::todo::TodoIdResponse,
        routes::app_routes
    };
    use todo_server::todo_api_web::model::http::Clients;
    use actix_web::{
        test, App,
    };
    use serde_json::from_str;
    use dotenv::dotenv;

    use crate::helpers::read_json;

    #[actix_rt::test]
    async fn valid_todo_post() {
        dotenv().ok();
        let mut app = test::init_service(
            App::new()
                .data(Clients::new())
                .configure(app_routes)
        ).await;
    
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
    use todo_server::todo_api_web::{
        model::todo::{TodoCardsResponse},
        routes::app_routes
    };
    use todo_server::todo_api_web::model::http::Clients;
    use actix_web::{
        test, App,
        http::StatusCode,
    };
    use actix_service::Service;
    use serde_json::from_str;
    use dotenv::dotenv;

    use crate::helpers::{read_json, mock_get_todos};

    #[actix_rt::test]
    async fn test_todo_index_ok() {
        dotenv().ok();
        let mut app = test::init_service(
            App::new()
                .data(Clients::new())
                .configure(app_routes)
        ).await;

        let req = test::TestRequest::with_uri("/api/index").to_request();
    
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_todo_cards_count() {
        dotenv().ok();
        let mut app = test::init_service(
            App::new()
                .data(Clients::new())
                .configure(app_routes)
        ).await;
    
        let post_req = test::TestRequest::post()
            .uri("/api/create")
            .header("Content-Type", "application/json")
            .set_payload(read_json("post_todo.json").as_bytes().to_owned())
            .to_request();

        let _ = app.call(post_req).await.unwrap();
        let req = test::TestRequest::with_uri("/api/index").to_request();
        let resp = test::read_response(&mut app, req).await;

        let todo_cards: TodoCardsResponse = from_str(&String::from_utf8(resp.to_vec()).unwrap()).unwrap();
        assert_eq!(todo_cards.cards.len(), 1);
    }


    #[actix_rt::test]
    async fn test_todo_cards_with_value() {
        dotenv().ok();
        let mut app = test::init_service(
            App::new()
                .data(Clients::new())
                .configure(app_routes)
        ).await;
    
        let post_req = test::TestRequest::post()
            .uri("/api/create")
            .header("Content-Type", "application/json")
            .set_payload(read_json("post_todo.json").as_bytes().to_owned())
            .to_request();

        let _ = app.call(post_req).await.unwrap();
        let req = test::TestRequest::with_uri("/api/index").to_request();
        let resp = test::read_response(&mut app, req).await;

        let todo_cards: TodoCardsResponse = from_str(&String::from_utf8(resp.to_vec()).unwrap()).unwrap();
        assert_eq!(todo_cards.cards, mock_get_todos());
    }

}

#[cfg(test)]
mod  auth {
    use actix_web::{
        test, App,
        http::StatusCode,
    };
    use todo_server::todo_api_web::{
        routes::app_routes
    };
    use todo_server::todo_api_web::model::http::Clients;
    use dotenv::dotenv;
    use crate::helpers::{read_json};


    #[actix_rt::test]
    async fn signup_returns_created_status() {
        dotenv().ok();
        let mut app = test::init_service(
            App::new()
                .data(Clients::new())
                .configure(app_routes)
        ).await;
    
        let signup_req = test::TestRequest::post()
            .uri("/auth/signup")
            .header("Content-Type", "application/json")
            .set_payload(read_json("signup.json").as_bytes().to_owned())
            .to_request();

        let resp = test::call_service(&mut app,signup_req).await;
        println!("{:?}", resp);
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_rt::test]
    async fn login_returns_token() {
        dotenv().ok();
        let mut app = test::init_service(
            App::new()
                .data(Clients::new())
                .configure(app_routes)
        ).await;

        let login_req = test::TestRequest::post()
            .uri("/auth/login")
            .header("Content-Type", "application/json")
            .set_payload(read_json("signup.json").as_bytes().to_owned())
            .to_request();

        let resp_body = test::read_response(&mut app, login_req).await;

        let jwt: String = String::from_utf8(resp_body.to_vec()).unwrap();
        
        assert!(jwt.contains("token"));
    }
}