use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
	pub content: String,
}

#[function_component(TableHeadCell)]
pub fn table_head_cell(props: &Props) -> Html {
	html!(
		<th scope="col" class="text-sm font-medium text-gray-900 px-6 py-4 text-left">
				{props.content.clone()}
	  </th>
	)
}
