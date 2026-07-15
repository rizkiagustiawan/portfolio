use leptos::prelude::*;

/// Hero section — primary profile introduction.
#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="hero" id="hero" aria-label="Introduction">
            <div class="hero__bg" aria-hidden="true">
                <div class="hero__grid-overlay"></div>
                <div class="hero__gradient"></div>
            </div>

            <header class="hero__content">
                <div class="hero__status">
                    <span class="hero__status-dot" aria-hidden="true"></span>
                    <span class="hero__status-text">"Sumbawa, Indonesia"</span>
                    <span class="hero__status-separator" aria-hidden="true">"·"</span>
                    <span class="hero__status-coord" aria-label="Coordinates">"08.5°S 117.4°E"</span>
                </div>

                <h1 class="hero__name">
                    "RIZKI AGUSTIAWAN"
                    <span class="hero__degree" aria-label="Bachelor of Engineering">", S.T."</span>
                </h1>

                <p class="hero__title">
                    "Environmental Engineer"
                    <span class="hero__title-separator" aria-hidden="true">"|"</span>
                    "GIS, Remote Sensing & ESG Analytics"
                </p>

                <p class="hero__summary">
                    "Environmental Engineer specializing in GIS, Remote Sensing, and ESG Analytics. "
                    "Combining a unique operational background in distributed infrastructure with high-performance "
                    "Rust engineering to build production-grade spatial intelligence platforms and accelerate "
                    "Indonesia's Net Zero 2060 transition through automated data-driven AI compliance systems."
                </p>

                <nav class="hero__actions" aria-label="Primary actions">
                    <a
                        href="https://github.com/rizkiagustiawan"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="hero__btn hero__btn--primary"
                        id="hero-github-btn"
                        aria-label="GitHub Profile (opens in a new tab)"
                    >
                        <svg aria-hidden="true" class="hero__btn-icon" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"></path><path d="M9 18c-4.51 2-5-2-7-2"></path></svg>
                        "GitHub"
                    </a>
                    <a
                        href="https://linkedin.com/in/rizkiagustiawan"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="hero__btn hero__btn--secondary"
                        id="hero-linkedin-btn"
                        aria-label="LinkedIn Profile (opens in a new tab)"
                    >
                        <svg aria-hidden="true" class="hero__btn-icon" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z"></path><rect width="4" height="12" x="2" y="9"></rect><circle cx="4" cy="4" r="2"></circle></svg>
                        "LinkedIn"
                    </a>
                    <a
                        href="#projects"
                        class="hero__btn hero__btn--ghost"
                        id="hero-projects-btn"
                    >
                        "View Missions ↓"
                    </a>
                </nav>

                <div class="hero__metrics" aria-label="Key Metrics">
                    <div class="hero__metric">
                        <span class="hero__metric-value">"5+"</span>
                        <span class="hero__metric-label">"Production Platforms"</span>
                    </div>
                    <div class="hero__metric">
                        <span class="hero__metric-value">"5"</span>
                        <span class="hero__metric-label">"Years Infrastructure Ops"</span>
                    </div>
                    <div class="hero__metric">
                        <span class="hero__metric-value">"NZ2060"</span>
                        <span class="hero__metric-label">"Indonesia Net Zero Target"</span>
                    </div>
                </div>
            </header>
        </section>
    }.into_any()
}
