use yew::prelude::*;
use yewdux::prelude::use_store;
use crate::store::{set_selected, Store};
use crate::components::table::{TableCell, TableHeadCell};

#[derive(Debug, Clone, PartialEq)]
pub struct Data {
	pub id: String,
	pub first_name: String,
	pub last_name: String,
	pub age: String,
}

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
	pub data: Vec<Data>,
}

#[function_component(Table)]
pub fn table(props: &Props) -> Html {
	let data = props.data.clone();

	let (store, dispatch) = use_store::<Store>();
	let selected = store.selected.clone();

	let rows_render = data
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
					<tr key={row.id.clone()} onclick={on_row_select} class={format!("border-b cursor-pointer duration-200 {}", if selected == row.id.clone() { "bg-green-500" } else if i % 2 != 0 { "bg-white" } else { "bg-gray-100" })}>
					  <TableCell content={row.id.clone()} />
					  <TableCell content={row.first_name} />
						<TableCell content={row.last_name} />
						<TableCell content={row.age} />
					</tr>
			}
		})
		.collect::<Html>();

	html!(
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
	)
}