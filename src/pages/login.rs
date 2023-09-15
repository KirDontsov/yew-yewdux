use yew::prelude::*;
use yewdux::prelude::*;
use web_sys::HtmlInputElement;
use uuid::Uuid;
use crate::store::{set_loading, set_show_alert, Credentials, Store, set_credentials};
use crate::utils::{get_input_val, get_input_val_len};
use crate::components::alert::{AlertComponent, Props as AlertProps};

#[function_component(Login)]
pub fn login() -> Html {
	let (store, dispatch) = use_store::<Store>();
	let loading = store.loading;
	let credentials = store.credentials.clone();
	let message = store.alert_input.alert_message.clone();

	let login = use_state(String::new);
	let password = use_state(String::new);

	let login_message = use_state(|| Option::<String>::None);
	let password_message = use_state(|| Option::<String>::None);

	let show_alert = store.alert_input.show_alert;
	let loading = store.loading;

	let alert_props = AlertProps {
		message: String::new(),
		delay_ms: 3000,
	};

	let login_input_ref = use_node_ref();
	let password_input_ref = use_node_ref();

	let handle_login_input = {
		let login = login.clone();
		let login_message = login_message.clone();
		Callback::from(move |event: InputEvent| {
			let value = get_input_val(event);
			login.set(value);
			if login_message.is_some() {
				login_message.set(None);
			}
		})
	};
	/* Validation */
	let handle_login_blur = {
		let login_message = login_message.clone();
		Callback::from(move |event: FocusEvent| {
			let len = get_input_val_len(event);
			if len > 0 && len < 5 {
				login_message.set(Some("Логин должен быть длиннее 5 символов".to_string()));
			}
		})
	};

	let handle_password_input = {
		let password = password.clone();
		let password_message = password_message.clone();
		Callback::from(move |event: InputEvent| {
			let value = get_input_val(event);
			password.set(value);
			if password_message.is_some() {
				password_message.set(None);
			}
		})
	};
	/* Validation */
	let handle_password_blur = {
		let password_message = password_message.clone();
		Callback::from(move |event: FocusEvent| {
			let len = get_input_val_len(event);
			if len > 0 && len < 5 {
				password_message.set(Some("Пароль должен быть длиннее 5 символов".to_string()));
			}
		})
	};

	let is_valid_login = login.trim().len() != 0 && login.trim().len() > 5 && login_message.is_none();
	let is_valid_password = password.trim().len() != 0 && password.trim().len() > 5 && password_message.is_none();

	let on_submit = {
		let cloned_dispatch = dispatch.clone();
		let login = login.clone();
		let password = password.clone();
		let login_message = login_message.clone();
		let password_message = password_message.clone();
		let login_input_ref = login_input_ref.clone();
		let password_input_ref = password_input_ref.clone();

		Callback::from(move |event: SubmitEvent| {
			event.prevent_default();
			set_loading(true, cloned_dispatch.clone());

			if !is_valid_login || !is_valid_password {
				if !is_valid_login {
					login_message.set(Some("Логин должен быть длиннее 5 символов".to_string()));
					set_loading(false, cloned_dispatch.clone());
					return;
				} else {
					password_message.set(Some("Пароль должен быть длиннее 5 символов".to_string()));
					set_loading(false, cloned_dispatch.clone());
					return;
				}
			}

			let new_credentials = Credentials {
				id: Uuid::new_v4(),
				login: login.to_string(),
				password: password.to_string(),
			};

			set_credentials(new_credentials, cloned_dispatch.clone());
			set_show_alert(
				"Реквизиты для входа добавлены успешно".to_string(),
				cloned_dispatch.clone(),
			);

			let text_input = login_input_ref.cast::<HtmlInputElement>().unwrap();
			let password_input = password_input_ref.cast::<HtmlInputElement>().unwrap();
			text_input.set_value("");
			password_input.set_value("");
			login.set(String::new());
			password.set(String::new());
			login_message.set(None);
			password_message.set(None);
			set_loading(false, cloned_dispatch.clone());
		})
	};


	html!(
		<div class="bg-gray-800 min-w-full h-full gap-8 p-16 flex flex-col items-center justify-center">
			if show_alert {
					<AlertComponent
						message={message.clone()}
						delay_ms={3000}
					 />
				}
			<form class="bg-gray-900 shadow-md rounded px-8 pt-6 pb-8 mb-4 flex flex-col gap-2 min-w-[700px]" onsubmit={on_submit}>
				<div class="mb-4">
				  <label class="block text-gray-300 text-sm font-bold mb-2" for="username">
					{"Логин"}
				  </label>
				  <input
						ref={login_input_ref}
						oninput={handle_login_input}
						onblur={handle_login_blur}
						class="bg-gray-700 shadow appearance-none rounded w-full py-2 px-3  text-gray-300 leading-tight focus:outline-none focus:shadow-outline"
						type="text"
						placeholder="Логин"
					/>
					{if let Some(msg) = login_message.as_ref() {
						html! { <div class="text-xs text-red-400">{msg.clone()}</div> }
					} else {
						html! {}
					}}
				</div>
				<div class="mb-6">
				  <label class="block text-gray-300 text-sm font-bold mb-2" for="password">
					{ "Пароль"}
				  </label>
				  <input
						ref={password_input_ref}
						oninput={handle_password_input}
						onblur={handle_password_blur}
						class="bg-gray-700 shadow appearance-none rounded w-full py-2 px-3 text-gray-300 leading-tight focus:outline-none focus:shadow-outline mb-2"
						type="password"
						placeholder="******************"
					/>
					{if let Some(msg) = password_message.as_ref() {
						html! { <div class="text-xs text-red-400">{msg.clone()}</div> }
					} else {
						html! {<p class="text-gray-400 text-xs">{"Придумайте надежный пароль содержащий спец. символы и цифры"}</p>}
					}}
				</div>
				<div class="flex items-center justify-between">

				<button
					type="submit"
					class={format!(
						"bg-blue-500 hover:bg-blue-700 disabled:bg-gray-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline  {}",
						if loading { "bg-[#ccc] text-gray-800"} else {"bg-indigo-600 text-white"}
					)}
					disabled={loading || !is_valid_login  || !is_valid_password}
				>
					{"Зарегестрироваться"}
				  </button>
				  <a class="inline-block align-baseline font-bold text-sm text-blue-500 hover:text-blue-800" href="#">
					{ "Уже есть аккаунт?"}
				  </a>
				</div>
			  </form>
			  <p class="text-center text-gray-500 text-xs">
				{"2020 Paradox Corp. All rights reserved."}
			  </p>
		</div>
	)
}
