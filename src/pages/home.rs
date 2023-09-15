use crate::components::table::{Data, Table};
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
	let rows: Vec<Data> = vec![
		Data {
			id: "1".into(),
			first_name: "John".into(),
			last_name: "Harrys".into(),
			age: "20".into(),
		},
		Data {
			id: "2".into(),
			first_name: "Kirill".into(),
			last_name: "Dontsov".into(),
			age: "33".into(),
		},
		Data {
			id: "3".into(),
			first_name: "FFF".into(),
			last_name: "FFF".into(),
			age: "44".into(),
		},
		Data {
			id: "4".into(),
			first_name: "AAA".into(),
			last_name: "AAA".into(),
			age: "55".into(),
		},
		Data {
			id: "5".into(),
			first_name: "DDD".into(),
			last_name: "DDD".into(),
			age: "66".into(),
		},
	];

	html!(
		<div class="min-w-full flex flex-col gap-8 p-16">
			<div class="text-white">
				<h1>{ "Home" }</h1>
			</div>

			<Table data={rows} />
		</div>
	)
}
