use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "local", storage_tab_sync)]
pub struct Store {
	pub feedbacks: Vec<Feedback>,
	pub credentials: Vec<Credentials>,
	pub loading: bool,
	pub alert_input: AlertInput,
	pub selected: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Credentials {
	pub id: uuid::Uuid,
	pub login: String,
	pub password: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Feedback {
	pub id: uuid::Uuid,
	pub text: String,
	pub rating: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct AlertInput {
	pub show_alert: bool,
	pub alert_message: String,
}

pub fn set_feedback(feedback: Feedback, dispatch: Dispatch<Store>) {
	dispatch.reduce_mut(move |store| {
		store.feedbacks.insert(0, feedback);
	})
}

pub fn delete_feedback(id: uuid::Uuid, dispatch: Dispatch<Store>) {
	dispatch.reduce_mut(move |store| {
		store.feedbacks.retain(|f| f.id != id);
	})
}

pub fn set_loading(loading: bool, dispatch: Dispatch<Store>) {
	dispatch.reduce_mut(move |store| {
		store.loading = loading;
	})
}

pub fn set_show_alert(message: String, dispatch: Dispatch<Store>) {
	dispatch.reduce_mut(move |store| {
		store.alert_input = AlertInput {
			alert_message: message,
			show_alert: true,
		};
	})
}

pub fn set_hide_alert(dispatch: Dispatch<Store>) {
	dispatch.reduce_mut(move |store| {
		store.alert_input.show_alert = false;
	})
}

pub fn set_selected(id: String, dispatch: Dispatch<Store>) {
	dispatch.reduce_mut(move |store| {
		store.selected = id;
	})
}

pub fn set_credentials(credentials: Credentials, dispatch: Dispatch<Store>) {
	dispatch.reduce_mut(move |store| {
		store.credentials.insert(0, credentials);
	})
}
