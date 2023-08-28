use yew::prelude::*;


#[function_component(Home)]
pub fn home() -> Html {
	html! {
		<main class="md:container mt-24 px-5">
			<h1 class="text-white">
				{ "Home" }
			</h1>
		</main>
	}
}