use leptos::prelude::*;

/// Individual project mission card.
#[component]
fn ProjectCard(
    /// Mission identifier label (e.g., "MISSION 01").
    mission: &'static str,
    /// Project title.
    title: &'static str,
    /// Project description.
    description: &'static str,
    /// Technology stack summary.
    stack: &'static str,
    /// Optional live deployment URL.
    #[prop(optional)]
    live_url: Option<&'static str>,
    /// Repository URL.
    repo_url: &'static str,
) -> impl IntoView {
    view! {
        <article class="project-card">
            <div class="project-card__header">
                <span class="project-card__mission">{mission}</span>
                <span class="project-card__indicator"></span>
            </div>
            <h3 class="project-card__title">{title}</h3>
            <p class="project-card__desc">{description}</p>
            <div class="project-card__stack">
                <span class="project-card__stack-label">"STACK"</span>
                <span class="project-card__stack-value">{stack}</span>
            </div>
            <div class="project-card__links">
                {live_url.map(|url| view! {
                    <a
                        href=url
                        target="_blank"
                        rel="noopener noreferrer"
                        class="project-card__link project-card__link--live"
                    >
                        "Live ↗"
                    </a>
                })}
                <a
                    href=repo_url
                    target="_blank"
                    rel="noopener noreferrer"
                    class="project-card__link project-card__link--repo"
                >
                    "Repository ↗"
                </a>
            </div>
        </article>
    }
}

/// Projects section — environmental intelligence mission cards.
#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <section class="projects" id="projects">
            <div class="section__header">
                <span class="section__label">"MISSION LOG"</span>
                <h2 class="section__title">"Environmental Intelligence Platforms"</h2>
                <p class="section__subtitle">
                    "Production-grade satellite, geospatial, and ESG systems — designed, built, and deployed independently."
                </p>
            </div>
            <div class="projects__grid">
                <ProjectCard
                    mission="MISSION 01"
                    title="Geo ESG A.E.C.O"
                    description="Greenwashing detection using Sentinel satellite systems. Automated ESG Compliance Observer with computer vision pipeline."
                    stack="Python, Rust, R, U-Net CV, scikit-learn, Shiny, matplotlib, Serde, Docker"
                    repo_url="https://github.com/rizkiagustiawan/GeoESG-Final"
                />
                <ProjectCard
                    mission="MISSION 02"
                    title="Air Quality Web GIS"
                    description="ISPU engine implementing PP 22/2021 regulation and atmospheric dispersion analysis for air quality monitoring."
                    stack="FastAPI, AERMOD/CALPUFF, Leaflet"
                    live_url="https://airtestingquality-jyzl.vercel.app/"
                    repo_url="https://github.com/rizkiagustiawan/airtestingquality"
                />
                <ProjectCard
                    mission="MISSION 03"
                    title="NTB Groundwater Monitor"
                    description="NASA GRACE groundwater intelligence platform integrating 280 ESDM monitoring wells for West Nusa Tenggara."
                    stack="AWS EC2, PostGIS, PostgreSQL, Docker, Chart.js"
                    repo_url="https://github.com/rizkiagustiawan/ntb-groundwater-monitor"
                />
                <ProjectCard
                    mission="MISSION 04"
                    title="Terra-Revive Sumbawa Island"
                    description="Geospatial environmental core infrastructure for vegetation recovery monitoring and land stability analysis."
                    stack="GDAL, OpenSSL, libgdal-dev"
                    repo_url="https://github.com/rizkiagustiawan/geoesg-sumbawa-core"
                />
                <ProjectCard
                    mission="MISSION 05"
                    title="Geo NTB Flood AI"
                    description="Flood risk intelligence and environmental reporting platform with multisensor fusion and ML-based prediction."
                    stack="FastAPI, Celery, Redis, Rust (PyO3), XGBoost, GDAL, Rasterio, Docker"
                    live_url="https://aeco.rizkiagustiawan.tech"
                    repo_url="https://github.com/rizkiagustiawan/geo-ntb-flood-ai"
                />
            </div>
        </section>
    }
}
