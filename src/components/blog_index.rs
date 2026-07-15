use crate::server::blog_api::get_posts;
use leptos::prelude::*;
use leptos_meta::{Meta, Title};

#[component]
pub fn BlogIndex() -> impl IntoView {
    let posts = Resource::new(
        || (),
        |_| async move { get_posts().await.unwrap_or_default() },
    );

    view! {
        <Title text="Engineering Log | Rizki Agustiawan"/>
        <Meta name="description" content="Technical blog covering Rust, geospatial engineering, and environmental intelligence systems."/>

        <div class="page-wrapper">
            <crate::app::Nav/>
            <main class="blog-container" style="max-width: 800px; margin: 0 auto; padding: 120px 2rem 4rem;">
                <header class="section__header" style="margin-bottom: 4rem;">
                    <span class="section__label">"ENGINEERING LOG"</span>
                    <h1 class="section__title">"Technical Transmissions"</h1>
                    <p class="section__subtitle">"Thoughts on Rust, geospatial systems, and environmental infrastructure."</p>
                </header>

                <Suspense fallback=|| view! { <p class="loading">"Decrypting logs..."</p> }>
                    <ul class="blog-list" style="list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 2rem;">
                        {move || posts.get().map(|posts| {
                            posts.into_iter().map(|post| {
                                view! {
                                    <li class="blog-card" style="border-bottom: 1px solid rgba(255, 255, 255, 0.1); padding-bottom: 2rem;">
                                        <time datetime=post.meta.date.clone() style="font-family: 'JetBrains Mono', monospace; font-size: 0.875rem; color: #8b949e; display: block; margin-bottom: 0.5rem;">
                                            {post.meta.date.clone()}
                                        </time>
                                        <h2 style="font-size: 1.5rem; font-weight: 500; margin-bottom: 0.5rem; font-family: 'JetBrains Mono', monospace;">
                                            <a href=format!("/blog/{}", post.slug) style="color: #c9d1d9; text-decoration: none;" onmouseover="this.style.color='#00ffcc'" onmouseout="this.style.color='#c9d1d9'">
                                                {post.meta.title}
                                            </a>
                                        </h2>
                                        <p style="color: #8b949e; font-size: 1rem; line-height: 1.6; margin-bottom: 1rem;">
                                            {post.meta.description}
                                        </p>
                                        <div class="tags" style="display: flex; gap: 0.5rem;">
                                            {post.meta.tags.into_iter().map(|tag| {
                                                view! {
                                                    <span style="font-size: 0.75rem; border: 1px solid rgba(255,255,255,0.1); padding: 0.1rem 0.5rem; border-radius: 4px; color: #8b949e;">
                                                        {tag}
                                                    </span>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </li>
                                }
                            }).collect_view()
                        })}
                    </ul>
                </Suspense>
            </main>
            <crate::app::Footer/>
        </div>
    }
}
