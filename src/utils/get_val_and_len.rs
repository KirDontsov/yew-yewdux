use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use yew::prelude::*;

pub fn get_input_val(event: InputEvent) -> String {
	let target = event.target().unwrap();
	let value = target.unchecked_into::<HtmlInputElement>().value();
	value
}

pub fn get_input_val_len(event: FocusEvent) -> usize {
	let target = event.target().unwrap();
	let value = target.unchecked_into::<HtmlInputElement>().value();
	let len = value.trim().len();
	len
}