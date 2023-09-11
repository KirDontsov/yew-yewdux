use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
	pub content: String,
}

#[function_component(TableCell)]
pub fn table_cell(props: &Props) -> Html {
	html!(
		<td scope="col" class="text-sm font-medium text-gray-900 px-6 py-4 text-left">
				{props.content.clone()}
	  </td>
	)
}
