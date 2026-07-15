use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    ParamSegment, StaticSegment,
};

use crate::components::blog_index::BlogIndex;
use crate::components::blog_post::BlogPost;
use crate::components::hero::Hero;
use crate::components::projects::Projects;
use crate::components::skills::Skills;
use crate::components::timeline::Timeline;

/// Shell function that wraps the entire HTML document for SSR.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" dir="ltr">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="theme-color" content="#0d1117"/>
                <meta name="robots" content="index, follow, max-image-preview:large, max-snippet:-1, max-video-preview:-1"/>
                <link rel="icon" type="image/svg+xml" href="/favicon.svg" />
                <link rel="preload" href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" as_="style" />
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
        <Title text="Rizki Agustiawan | Environmental Engineer | GIS, Remote Sensing & ESG Analytics"/>
        <Meta
            name="description"
            content="Portfolio of Rizki Agustiawan, S.T. | Environmental Engineer specializing in ESG intelligence systems, geospatial infrastructure, and climate technology built with Rust."
        />
        <Meta name="author" content="Rizki Agustiawan"/>
        <Meta
            name="keywords"
            content="Environmental Engineer, ESG, Geospatial, Rust, WebAssembly, Climate Tech, Indonesia, Rizki Agustiawan, S.T., Sumbawa"
        />
        <link rel="canonical" href="https://rizkiagustiawan.tech/"/>

        // Open Graph
        <Meta property="og:title" content="Rizki Agustiawan | Environmental Engineer | GIS, Remote Sensing & ESG Analytics"/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:url" content="https://rizkiagustiawan.tech/"/>
        <Meta
            property="og:description"
            content="Environmental intelligence systems, ESG automation, and geospatial infrastructure built with Rust."
        />
        <Meta property="og:site_name" content="Rizki Agustiawan Portfolio"/>
        <Meta property="og:image" content="https://rizkiagustiawan.tech/og-image.jpg"/>
        <Meta property="og:image:alt" content="Rizki Agustiawan Portfolio Preview"/>

        // Twitter Cards
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content="Rizki Agustiawan | Environmental Engineer"/>
        <Meta
            name="twitter:description"
            content="Environmental intelligence systems, ESG automation, and geospatial infrastructure built with Rust."
        />
        <Meta name="twitter:image" content="https://rizkiagustiawan.tech/og-image.jpg"/>

        // JSON-LD Structured Data
        <script type="application/ld+json">
            r#"{
              "@context": "https://schema.org",
              "@graph": [
                  {
                    "@type": "WebSite",
                    "@id": "https://rizkiagustiawan.tech/#website",
                    "url": "https://rizkiagustiawan.tech",
                    "name": "Rizki Agustiawan Portfolio",
                    "description": "Environmental intelligence systems, ESG automation, and geospatial infrastructure built with Rust.",
                    "publisher": {
                      "@id": "https://rizkiagustiawan.tech/#person"
                    }
                  },
                  {
                    "@type": "Person",
                    "@id": "https://rizkiagustiawan.tech/#person",
                    "name": "Rizki Agustiawan",
                    "honorificSuffix": "S.T.",
                    "jobTitle": "Environmental Engineer",
                    "url": "https://rizkiagustiawan.tech",
                    "image": "https://rizkiagustiawan.tech/og-image.jpg",
                    "sameAs": [
                      "https://github.com/rizkiagustiawan",
                      "https://linkedin.com/in/rizkiagustiawan",
                      "https://www.facebook.com/Awan3x",
                      "https://www.instagram.com/awan3x/"
                    ],
                    "alumniOf": {
                      "@type": "CollegeOrUniversity",
                      "name": "Universitas Teknologi Sumbawa"
                    },
                    "knowsAbout": ["Environmental Engineering", "ESG", "Rust", "WebAssembly", "Geospatial", "Climate Tech"]
                  }
              ]
            }"#
        </script>

        <Router>
            <Routes fallback=|| view! { <NotFound/> }.into_any()>
                <Route path=StaticSegment("") view=HomePage/>
                <Route path=StaticSegment("blog") view=BlogIndex/>
                <Route path=(StaticSegment("blog"), ParamSegment("slug")) view=BlogPost/>
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
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="nav" id="main-nav" aria-label="Main Navigation">
            <div class="nav__inner">
                <a href="/" class="nav__logo" aria-label="Home">
                    <span class="nav__logo-icon" aria-hidden="true">"◉"</span>
                </a>
                <ul class="nav__links" style="list-style: none; margin: 0; padding: 0;">
                    <li><a href="/#projects" class="nav__link">"Projects"</a></li>
                    <li><a href="/#timeline" class="nav__link">"Experience"</a></li>
                    <li><a href="/#skills" class="nav__link">"Skills"</a></li>
                    <li><a href="/blog" class="nav__link">"Blog"</a></li>
                </ul>
            </div>
        </nav>
    }
}

/// Site footer.
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer" id="footer" aria-label="Site Footer">
            <div class="footer__inner">
                <div class="footer__grid">
                    <div class="footer__col">
                        <p class="footer__tagline">
                            "Environmental Intelligence Systems"
                        </p>
                        <div style="display: flex; align-items: center; gap: 0.5rem; margin-top: 1rem; font-size: 0.75rem; font-family: 'JetBrains Mono', monospace; color: #8b949e;">
                            <span style="display: inline-block; width: 6px; height: 6px; border-radius: 50%; background-color: #3fb950; box-shadow: 0 0 8px #3fb950;"></span>
                            "SYSTEMS OPERATIONAL"
                        </div>
                    </div>
                    <div class="footer__col">
                        <p class="footer__heading" id="footer-connect-heading">"Connect"</p>
                        <ul aria-labelledby="footer-connect-heading" style="list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: var(--spacing-sm);">
                            <li>
                                <a
                                    href="https://www.facebook.com/Awan3x"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="footer__link"
                                    aria-label="Facebook Profile (opens in a new tab)"
                                >
                                    "Facebook"
                                </a>
                            </li>
                            <li>
                                <a
                                    href="https://www.instagram.com/awan3x/"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="footer__link"
                                    aria-label="Instagram Profile (opens in a new tab)"
                                >
                                    "Instagram"
                                </a>
                            </li>
                            <li>
                                <a
                                    href="https://t.me/awan3x"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="footer__link"
                                    aria-label="Telegram Profile (opens in a new tab)"
                                >
                                    "Telegram"
                                </a>
                            </li>
                            <li>
                                <a
                                    href="mailto:rizki.agustiawan740@gmail.com"
                                    class="footer__link"
                                    aria-label="Email Rizki Agustiawan"
                                >
                                    "Email"
                                </a>
                            </li>
                        </ul>
                    </div>
                    <div class="footer__col">
                        <p class="footer__heading">"Core Architecture"</p>
                        <ul style="list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 0.4rem; font-size: 0.82rem; color: var(--c-text-secondary);">
                            <li>"• Rust (Axum)"</li>
                            <li>"• Leptos SSR / WebAssembly"</li>
                            <li>"• PostGIS & GDAL"</li>
                            <li>"• Multi-Agent AI (CrewAI)"</li>
                        </ul>
                    </div>
                </div>
                <div class="footer__bottom">
                    <p class="footer__copy">
                        "© 2026 Rizki Agustiawan. All rights reserved."
                    </p>
                    <p class="footer__note">
                        "Built with Rust & Leptos SSR | Rendered on the server, hydrated on the client."
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
        <div class="page-wrapper">
            <Nav/>
            <main style="display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 70vh; text-align: center; padding: 2rem;">
                <h1 style="font-size: 5rem; font-family: 'JetBrains Mono', monospace; color: var(--c-accent); margin-bottom: 1rem;">"404"</h1>
                <p style="font-size: 1.25rem; color: var(--c-text-secondary); margin-bottom: 2rem;">"AREA NOT MAPPED. The coordinates you requested do not exist in this sector."</p>
                <a href="/" class="hero__btn hero__btn--ghost">"← Return to Base"</a>
            </main>
            <Footer/>
        </div>
    }
}
