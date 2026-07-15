use crate::server::blog_api::get_post;
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_params_map;

#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|p| p.get("slug").unwrap_or_default());

    let post = Resource::new(slug, |s| async move { get_post(s).await.unwrap_or(None) });

    view! {
        <div class="page-wrapper">
            <crate::app::Nav/>

            <main class="blog-post-container" style="max-width: 800px; margin: 0 auto; padding: 120px 2rem 4rem;">
                <Suspense fallback=|| view! { <p class="loading">"Loading structural data..."</p> }>
                    {move || post.get().map(|opt_post| {
                        match opt_post {
                            Some(p) => {
                                let meta_title = format!("{} | Rizki Agustiawan", p.meta.title);
                                let json_ld = format!(
                                    r#"{{
                                      "@context": "https://schema.org",
                                      "@type": "BlogPosting",
                                      "headline": "{}",
                                      "datePublished": "{}",
                                      "author": {{
                                        "@type": "Person",
                                        "name": "Rizki Agustiawan"
                                      }},
                                      "description": "{}"
                                    }}"#,
                                    p.meta.title, p.meta.date, p.meta.description
                                );

                                view! {
                                    <Title text=meta_title />
                                    <Meta name="description" content=p.meta.description.clone() />
                                    <script type="application/ld+json">{json_ld}</script>

                                    <article class="blog-article">
                                        <header style="margin-bottom: 3rem; border-bottom: 1px solid rgba(255,255,255,0.1); padding-bottom: 2rem;">
                                            <a href="/blog" style="color: #8b949e; text-decoration: none; font-size: 0.875rem; margin-bottom: 1.5rem; display: inline-block;">"← Back to Log"</a>
                                            <h1 style="font-size: 2.5rem; font-weight: 600; font-family: 'JetBrains Mono', monospace; margin-bottom: 1rem; line-height: 1.2;">
                                                {p.meta.title}
                                            </h1>
                                            <div style="display: flex; gap: 1rem; align-items: center; color: #8b949e; font-size: 0.875rem; font-family: 'JetBrains Mono', monospace;">
                                                <time datetime=p.meta.date.clone()>{p.meta.date.clone()}</time>
                                                <span>"·"</span>
                                                <div style="display: flex; gap: 0.5rem;">
                                                    {p.meta.tags.into_iter().map(|tag| view! { <span>{format!("#{}", tag)}</span> }).collect_view()}
                                                </div>
                                            </div>
                                        </header>

                                        <div class="blog-content" style="line-height: 1.8; font-size: 1.125rem; color: #c9d1d9;" inner_html=p.content_html>
                                        </div>
                                    </article>
                                }.into_any()
                            },
                            None => view! {
                                <div class="not-found">
                                    <h1>"404"</h1>
                                    <p>"Log entry not found."</p>
                                    <a href="/blog">"← Return to Log"</a>
                                </div>
                            }.into_any()
                        }
                    })}
                </Suspense>
            </main>

            <crate::app::Footer/>
        </div>
    }
}
