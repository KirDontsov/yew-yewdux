use yew::prelude::*;
use yewdux::prelude::*;

use super::feedback_item::FeedbackItem;
use crate::store::{set_loading, Store};

#[function_component(FeedbackList)]
pub fn feedback_list() -> Html {
	let (store, dispatch) = use_store::<Store>();
	let feedback_list = store.feedbacks.clone();
	let loading = store.loading.clone();

	let on_toggle = {
		Callback::from(move |_: MouseEvent| {
			set_loading(!loading, dispatch.clone());
		})
	};

	html! {
		<div>
			{
				feedback_list.into_iter().map(|feedback|{
					let key = feedback.id.to_string();
					html!{<FeedbackItem {key} feedback={feedback.clone()} />}
				}).collect::<Html>()
			}
		<button
			onclick={on_toggle}
					type="submit"
					class={format!(
						"border-0 rounded-md w-28 h-10 cursor-pointer hover:bg-indigo-500 mb-4 {}",
						if loading { "bg-[#ccc] text-gray-800"} else {"bg-indigo-600 text-white"}
					)}
				>
					{"Toggle"}
				</button>
		</div>
	}
}
