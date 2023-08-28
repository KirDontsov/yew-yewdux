use yew::prelude::*;


#[function_component(NotFound)]
pub fn not_found() -> Html {
	html! {
		<main class="md:container mt-24 px-5">
			<h1 class="text-white">
				{ "404 Not Found" }
			</h1>
		</main>
	}
}