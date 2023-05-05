use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::home::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>

        <Stylesheet href="https://unpkg.com/leaflet@1.9.3/dist/leaflet.css"/>
        <Script src="https://unpkg.com/leaflet@1.9.3/dist/leaflet.js"/>
        <Router>
            <Routes>
                <Route path="" view=  move |cx| view! { cx, <Home/> }/>
            </Routes>
        </Router>
    }
}
