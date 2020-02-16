mod ping_readiness {
    use todo_server::todo_api_web::routes::app_routes;

    use bytes::Bytes;
    use actix_web::{
        test, App,
        http::StatusCode,
    };
    use actix_service::Service;

    #[actix_rt::test]
    async fn test_ping_pong() {
        let mut app = test::init_service(
            App::new().configure(
                app_routes
            )
        ).await;

        let req = test::TestRequest::get()
            .uri("/ping")
            .to_request();
        let resp = test::read_response(&mut app, req).await;

        assert_eq!(resp, Bytes::from_static(b"pong"));
    }

    #[actix_rt::test]
    async fn test_readiness_ok() {
        let mut app = test::init_service(
            App::new()
                .configure(app_routes)
        ).await;
    
        let req = test::TestRequest::with_uri("/~/ready").to_request();
    
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::ACCEPTED);
    }
}

mod create_todo {
    use todo_server::todo_api_web::{
        model::todo::TodoIdResponse,
        routes::app_routes
    };

    use actix_web::{
        test, App,
    };
    use serde_json::from_str;

    use crate::helpers::read_json;

    #[actix_rt::test]
    async fn valid_todo_post() {
        let mut app = test::init_service(
            App::new()
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

mod read_all_todos {
    use todo_server::todo_api_web::{
        model::todo::{TodoCardsResponse},
        routes::app_routes
    };

    use actix_web::{
        test, App,
        http::StatusCode,
    };
    use actix_service::Service;
    use serde_json::from_str;

    use crate::helpers::{read_json, mock_get_todos};

    #[actix_rt::test]
    async fn test_todo_index_ok() {
        let mut app = test::init_service(
            App::new()
                .configure(app_routes)
        ).await;

        let req = test::TestRequest::with_uri("/api/index").to_request();
    
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_todo_cards_count() {
        let mut app = test::init_service(
            App::new()
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
        let mut app = test::init_service(
            App::new()
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

mod  auth {
    use actix_web::{
        test, App,
        http::StatusCode,
    };
    use actix_service::Service;
    use todo_server::todo_api_web::{
        routes::app_routes
    };
    use crate::helpers::{read_json};


    #[actix_rt::test]
    async fn signup_returns_created_status() {
        let mut app = test::init_service(
            App::new()
                .configure(app_routes)
        ).await;
    
        let signup_req = test::TestRequest::post()
            .uri("/auth/signup")
            .header("Content-Type", "application/json")
            .set_payload(read_json("signup.json").as_bytes().to_owned())
            .to_request();

        let resp = app.call(signup_req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);
    }
}