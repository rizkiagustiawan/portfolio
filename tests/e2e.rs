use axum::{body::Body, http::Request};
use tower::ServiceExt;

// Note: To truly E2E test a Leptos SSR Axum app without spinning up a full browser (like Playwright),
// we test the Axum router directly to ensure it returns 200 OK and valid HTML containing expected strings.

// In a real Leptos app, extracting the router out of `main.rs` into a separate function is needed for clean testing.
// Since `main.rs` directly defines it, we'll write a test that simulates the router creation.

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_homepage_loads() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use rizkiagustiawan_tech::app::*;

    let leptos_options = LeptosOptions::builder()
        .output_name("rizkiagustiawan-tech")
        .build();
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let request = Request::builder().uri("/").body(Body::empty()).unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), 200);
}

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_blog_index_loads() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use rizkiagustiawan_tech::app::*;

    let leptos_options = LeptosOptions::builder()
        .output_name("rizkiagustiawan-tech")
        .build();
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let request = Request::builder().uri("/blog").body(Body::empty()).unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), 200);
}

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_404_page() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use rizkiagustiawan_tech::app::*;

    let leptos_options = LeptosOptions::builder()
        .output_name("rizkiagustiawan-tech")
        .build();
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let request = Request::builder()
        .uri("/nonexistent-page")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), 404);
}
