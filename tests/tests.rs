#[cfg(test)]
mod tests {
    use actix_web::{App, web};
    use actix_web::test::{self, TestRequest};
    use sqlx::PgPool;

    use testcontainers_modules::{
        postgres::Postgres,
        testcontainers::runners::AsyncRunner,
    };
    
    // Import the API configuration function
    use recipe_api::api;

    #[actix_web::test]
    async fn it_tests_all_api_endpoints() {
    let node: testcontainers_modules::testcontainers::ContainerAsync<Postgres> = Postgres::default().start().await.expect("Failed to start Postgres container");;
                let host_port = node.get_host_port_ipv4(5432).await.expect("Failed to get host port");

    // prepare connection string
    let connection_string = &format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
       host_port
    );

        let pool = PgPool::connect(&connection_string)
            .await
            .expect("Failed to create SQLx pool");
        
        // Run migrations once
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run database migrations");

        // Initialize the actix-web test service once
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(api::config),
        ).await;

        // Test the /api/recipes endpoint
        let req1 = TestRequest::get().uri("/api/recipes").to_request();
        let resp1 = test::call_service(&app, req1).await;

        assert!(resp1.status().is_success());
        let body1 = test::read_body(resp1).await;
        assert_eq!(body1, "[z]");

    }
}
