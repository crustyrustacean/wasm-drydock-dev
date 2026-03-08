// frontend/src/lib.rs

// dependencies
use crate::components::Version;
use yew::prelude::*;

// module declarations
mod components;
mod hooks;

// App component
#[component]
fn App() -> Html {
    html! {
        <div class="page">
            <header>
                <h1>{ "WASM Drydock" }</h1>
                <h2>{ "One command. The whole loop." }</h2>
                <p>{ "wasm-drydock is a single-command dev tool for fullstack Rust web applications. Scaffold a project, start the server, and stay focused on your code." }</p>
                <nav>
                    <a href="#get-started" class="btn-primary">{ "Get Started" }</a>
                    <a href="https://crustyrustacean.github.io/wasm-drydock" class="btn-secondary" target="_blank" rel="noreferrer">{ "Documentation" }</a>
                    <a href="https://github.com/crustyrustacean/wasm-drydock" class="btn-secondary" target="_blank" rel="noreferrer">{ "View on GitHub" }</a>
                </nav>
            </header>
            <main>
                <section>
                    <h3>{ "What it does" }</h3>
                    <p>{ "Most fullstack Rust setups ask you to manage two separate processes — one for your frontend WASM build, one for your backend server. wasm-drydock replaces both with a single command that owns the entire development loop." }</p>
                </section>
                <section>
                    <h3>{ "Features" }</h3>
                    <ul>
                        <li>{ "Scaffolds a three-crate Cargo workspace in seconds" }</li>
                        <li>{ "Builds your Yew frontend with wasm-pack on startup" }</li>
                        <li>{ "Proxies API requests to your Actix-web backend" }</li>
                        <li>{ "Watches source files and rebuilds automatically" }</li>
                        <li>{ "Recompiles SCSS on the fly" }</li>
                        <li>{ "Pushes build errors directly to the browser" }</li>
                        <li>{ "Produces a single self-contained release binary" }</li>
                        <li>{ "Easy deployment to your own server or via Docker" }</li>
                    </ul>
                </section>
                <section id="get-started">
                    <h3>{ "Get started in three steps" }</h3>
                    <pre><code>{ "cargo install wasm-drydock\nwasm-drydock init my-app\ncd my-app && wasm-drydock dev" }</code></pre>
                    <p>{ "Open " }<code>{ "http://localhost:8080" }</code>{ ". Start building." }</p>
                </section>
            </main>
            <footer>
                <p>
                    { "Copyright 2026 Jeffery D. Mitchell | All rights reserved. | This site built with " }
                    <code>{ "wasm-drydock" }</code>
                </p>
                <p>{ "Site version: " }  <Version /> </p>
            </footer>
        </div>
    }
}

// application entry point
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
