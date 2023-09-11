use yew::prelude::*;
use yewdux::prelude::use_store;
use crate::store::{set_selected, Store};
use crate::components::table::TableCell;
use crate::components::table::TableHeadCell;

#[derive(Debug, Clone)]
pub struct Data {
	id: String,
	first_name: String,
	last_name: String,
	age: String,
}

pub struct HomeProps {
	data: Vec<Data>,
}

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

	let (store, dispatch) = use_store::<Store>();
	let selected = store.selected.clone();

	let rows_render = rows
		.into_iter()
		.enumerate()
		.map(|(i, row)| {
			let on_row_select = {
				let row = row.clone();
				let clonned_dispatch = dispatch.clone();
				Callback::from(move |_| {
					set_selected(row.id.clone(), clonned_dispatch.clone());
				})
			};

			html! {
					<tr key={row.id.clone()} onclick={on_row_select} class={format!("border-b cursor-pointer duration-300 {}", if selected == row.id.clone() { "bg-green-500" } else if i % 2 != 0 { "bg-white" } else { "bg-gray-100" })}>
					  <TableCell content={row.id.clone()} />
					  <TableCell content={row.first_name} />
						<TableCell content={row.last_name} />
						<TableCell content={row.age} />
					</tr>
			}
		})
		.collect::<Html>();

	html!(
		<div class="min-w-full flex flex-col gap-8 p-16">
			<div class="text-white">
				<h1>{ "Home" }</h1>
			</div>

			<table class="min-w-full">
			<thead class="bg-white border-b">
					<tr>
						<TableHeadCell content="#" />
						<TableHeadCell content="First" />
						<TableHeadCell content="Last" />
						<TableHeadCell content="Age" />
					</tr>
			</thead>
		  <tbody>
				{rows_render}
		  </tbody>
		</table>
		</div>
	)
}
