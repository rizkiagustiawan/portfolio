use leptos::prelude::*;

/// Individual timeline entry.
#[component]
fn TimelineEntry(
    /// Date range string.
    period: &'static str,
    /// Role/title.
    role: &'static str,
    /// Description of responsibilities.
    description: &'static str,
    /// Whether this is the current/active role.
    #[prop(default = false)]
    active: bool,
) -> impl IntoView {
    let class = if active {
        "timeline__entry timeline__entry--active"
    } else {
        "timeline__entry"
    };

    view! {
        <div class=class>
            <div class="timeline__marker">
                <div class="timeline__dot"></div>
                <div class="timeline__line"></div>
            </div>
            <div class="timeline__content">
                <span class="timeline__period">{period}</span>
                <h3 class="timeline__role">{role}</h3>
                <p class="timeline__desc">{description}</p>
            </div>
        </div>
    }
}

/// Experience timeline section.
#[component]
pub fn Timeline() -> impl IntoView {
    view! {
        <section class="timeline-section" id="timeline">
            <div class="section__header">
                <span class="section__label">"OPERATIONAL HISTORY"</span>
                <h2 class="section__title">"Professional Experience"</h2>
                <p class="section__subtitle">
                    "A trajectory spanning environmental engineering, global supply chains, and distributed systems."
                </p>
            </div>
            <div class="timeline">
                <TimelineEntry
                    period="Mar 2026 – Present"
                    role="Environmental Technology Developer (Satellite & AI Systems)"
                    description="Built four production-grade satellite platforms independently. Implemented Multi AI Agent Systems (CrewAI) for automated workflows."
                    active=true
                />
                <TimelineEntry
                    period="Mar 2024 – Feb 2026"
                    role="Global Procurement & Scope 3 Supply Chain Analyst"
                    description="Managed cross-border procurement (China, US, Malaysia) with zero compliance violations."
                />
                <TimelineEntry
                    period="Mar 2021 – Feb 2026"
                    role="ESG Technology Researcher & ReFi Protocol Engineer"
                    description="Operated distributed node infrastructure for 5 years. Evaluated ReFi protocols for carbon credit verification."
                />
                <TimelineEntry
                    period="Sep 2024 – Nov 2024"
                    role="Environmental Engineer Intern — Dinas PUPR Provinsi NTB"
                    description="Supervised leachate containment construction and conducted GIS compliance audits."
                />
            </div>
        </section>
    }
}
