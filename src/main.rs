mod components;
mod pages;
mod routing;
mod store;
mod utils;

use routing::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
	html! {
		<BrowserRouter>
			<Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
		</BrowserRouter>
	}
}

fn main() { yew::Renderer::<App>::new().render(); }
