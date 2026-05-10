use leptos::prelude::*;

/// A category card for grouping skills.
#[component]
fn SkillCategory(
    /// Category title.
    title: &'static str,
    /// Icon character.
    icon: &'static str,
    /// Skills list as a single string.
    skills: &'static str,
) -> impl IntoView {
    view! {
        <div class="skill-category">
            <div class="skill-category__header">
                <span class="skill-category__icon">{icon}</span>
                <h3 class="skill-category__title">{title}</h3>
            </div>
            <p class="skill-category__skills">{skills}</p>
        </div>
    }
}

/// A certification badge.
#[component]
fn CertBadge(
    /// Certification name.
    name: &'static str,
) -> impl IntoView {
    view! {
        <span class="cert-badge">{name}</span>
    }
}

/// Skills & Certifications section.
#[component]
pub fn Skills() -> impl IntoView {
    view! {
        <section class="skills-section" id="skills">
            <div class="section__header">
                <span class="section__label">"CAPABILITY MATRIX"</span>
                <h2 class="section__title">"Skills & Certifications"</h2>
                <p class="section__subtitle">
                    "Core competencies across engineering, geospatial science, and environmental regulation."
                </p>
            </div>

            <div class="skills__grid">
                <SkillCategory
                    title="Engineering"
                    icon="⟐"
                    skills="Rust, Python, R, WebAssembly, FastAPI, Docker, PostgreSQL/PostGIS, AWS, Terraform, Redis, CI/CD, PyO3, Polars"
                />
                <SkillCategory
                    title="Geospatial & ESG"
                    icon="◎"
                    skills="Google Earth Engine, QGIS, STAC, GeoParquet, AERMOD/CALPUFF, ESG Reporting, Scope 3 Traceability"
                />
                <SkillCategory
                    title="Environmental Regulation"
                    icon="◇"
                    skills="PP No. 22/2021, AMDAL & UKL-UPL, ISO 14001:2015, PROPER KLHK, SIMPEL Reporting, UU No. 32/2009"
                />
            </div>

            <div class="skills__certs">
                <h3 class="skills__certs-title">"Certifications"</h3>
                <div class="skills__certs-grid">
                    <CertBadge name="BNSP Supervisor K3 Konstruksi"/>
                    <CertBadge name="SMK3 (UTS)"/>
                    <CertBadge name="ESG Reporting (CPD Accredited)"/>
                    <CertBadge name="NASA ARSET — Remote Sensing"/>
                    <CertBadge name="NASA ARSET — Developing Sustainable Earth Science App"/>
                    <CertBadge name="NASA ARSET — Groundwater Changes for Water Resources Management"/>
                    <CertBadge name="Multi AI Agent Systems (CrewAI)"/>
                </div>
            </div>
        </section>
    }
}
