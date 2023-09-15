use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
	pub content: String,
}

#[function_component(TableCell)]
pub fn table_cell(props: &Props) -> Html {
	let clipboard = use_clipboard();

	let onclick_write_text = {
		let clipboard = clipboard.clone();
		let clonned_content = props.content.clone();
		Callback::from(move |_| {
			clipboard.write_text(clonned_content.clone());
		})
	};

	html!(
		<td onclick={onclick_write_text} scope="col" class="text-sm font-medium text-gray-900 px-6 py-4 text-left">
				{props.content.clone()}
	  </td>
	)
}
