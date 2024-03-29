use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::{delete_feedback, set_show_alert, Feedback, Store};

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
	pub feedback: Feedback,
}

fn confirm_delete(message: &str) -> bool {
	web_sys::Window::confirm_with_message(&web_sys::window().unwrap(), message).unwrap()
}

#[function_component(FeedbackItem)]
pub fn feedback_item(props: &Props) -> Html {
	let (_, dispatch) = use_store::<Store>();

	let on_delete = {
		let feedback_id = props.feedback.id.clone();

		Callback::from(move |_: MouseEvent| {
			if confirm_delete("Do you really want to delete this item?") {
				delete_feedback(feedback_id, dispatch.clone());
				set_show_alert(
					"Feedback deleted successfully".to_string(),
					dispatch.clone(),
				);
			}
		})
	};

	html! {
		<div class="bg-white text-gray-700 rounded-lg p-8 my-5 relative">
			<div class="bg-pink-500 text-white rounded-full border-2 border-gray-200 w-12 h-12 flex items-center justify-center text-2xl font-bold absolute top-0 left-0 -mt-4 -ml-4">
				{props.feedback.rating}
			</div>
			<button class="absolute font-bold top-2 right-4 cursor-pointer bg-transparent border-none" onclick={on_delete}>{"X"}</button>
			<p>
				{&props.feedback.text}
			</p>
		</div>
	}
}
