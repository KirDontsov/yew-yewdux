use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::rating::Rating;
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
	#[at("/")]
	Home,
	#[at("/rating")]
	Rating,
	#[not_found]
	#[at("/404")]
	NotFound,
}

pub fn switch(routes: Route) -> Html {
	match routes {
		Route::Home => html! { <Home /> },
		Route::Rating => html! { <Rating /> },
		Route::NotFound => html! { <NotFound /> },
	}
}
