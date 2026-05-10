use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::components::hero::Hero;
use crate::components::projects::Projects;
use crate::components::skills::Skills;
use crate::components::timeline::Timeline;

/// Shell function that wraps the entire HTML document for SSR.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" type="image/svg+xml" href="/favicon.svg" />
                <link
                    rel="preconnect"
                    href="https://fonts.googleapis.com"
                />
                <link
                    rel="preconnect"
                    href="https://fonts.gstatic.com"
                    crossorigin="anonymous"
                />
                <link
                    href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap"
                    rel="stylesheet"
                />
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

/// Root application component.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/rizkiagustiawan-tech.css"/>
        <Title text="Rizki Agustiawan — Environmental Engineer & ESG Intelligence Builder"/>
        <Meta
            name="description"
            content="Portfolio of Rizki Agustiawan, S.T. — Environmental Engineer specializing in ESG intelligence systems, geospatial infrastructure, and climate technology built with Rust."
        />
        <Meta name="author" content="Rizki Agustiawan"/>
        <Meta
            name="keywords"
            content="Environmental Engineer, ESG, Geospatial, Rust, WebAssembly, Climate Tech, Indonesia"
        />
        <Meta property="og:title" content="Rizki Agustiawan — Environmental Engineer & ESG Intelligence Builder"/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:url" content="https://rizkiagustiawan.tech"/>
        <Meta
            property="og:description"
            content="Environmental intelligence systems, ESG automation, and geospatial infrastructure — built with Rust."
        />

        <Router>
            <Routes fallback=|| view! { <NotFound/> }.into_any()>
                <Route path=StaticSegment("") view=HomePage/>
            </Routes>
        </Router>
    }
}

/// Main single-page portfolio layout.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="page-wrapper">
            <Nav/>
            <main>
                <Hero/>
                <Projects/>
                <Timeline/>
                <Skills/>
            </main>
            <Footer/>
        </div>
    }
    .into_any()
}

/// Top navigation bar.
#[component]
fn Nav() -> impl IntoView {
    view! {
        <nav class="nav" id="main-nav">
            <div class="nav__inner">
                <a href="/" class="nav__logo">
                    <span class="nav__logo-icon">"◉"</span>
                </a>
                <div class="nav__links">
                    <a href="#projects" class="nav__link">"Projects"</a>
                    <a href="#timeline" class="nav__link">"Experience"</a>
                    <a href="#skills" class="nav__link">"Skills"</a>
                </div>
            </div>
        </nav>
    }
}

/// Site footer.
#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="footer" id="footer">
            <div class="footer__inner">
                <div class="footer__grid">
                    <div class="footer__col">
                        <p class="footer__tagline">
                            "Environmental Intelligence Systems"
                        </p>
                    </div>
                    <div class="footer__col">
                        <p class="footer__heading">"Connect"</p>
                        <a
                            href="https://www.facebook.com/Awan3x"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="footer__link"
                        >
                            "Facebook"
                        </a>
                        <a
                            href="https://www.instagram.com/awan3x/"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="footer__link"
                        >
                            "Instagram"
                        </a>
                        <a
                            href="mailto:rizki.agustiawan740@gmail.com"
                            class="footer__link"
                        >
                            "Email"
                        </a>
                    </div>
                    <div class="footer__col">
                        <p class="footer__heading">"Stack"</p>
                        <p class="footer__text">"Rust · Leptos · Axum · WASM"</p>
                    </div>
                </div>
                <div class="footer__bottom">
                    <p class="footer__copy">
                        "© 2026 Rizki Agustiawan. All rights reserved."
                    </p>
                    <p class="footer__note">
                        "Built with Rust & Leptos SSR — Rendered on the server, hydrated on the client."
                    </p>
                </div>
            </div>
        </footer>
    }
}

/// 404 page.
#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="not-found">
            <h1>"404"</h1>
            <p>"Page not found."</p>
            <a href="/">"← Return to Observatory"</a>
        </div>
    }
}
