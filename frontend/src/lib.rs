use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, HtmlInputElement, MessageEvent, WebSocket};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let ws = WebSocket::new("ws://127.0.0.1:8080")?;
    let document = get_document();

    let username = prompt("Enter your name: ");

    // โซนแสดงผลข้อความ
    let messages = document.get_element_by_id("messages").unwrap();

    // --- จัดการเมื่อได้รับข้อความจาก server ---
    let messages_clone = messages.clone();
    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |event: MessageEvent| {
        if let Ok(txt) = event.data().dyn_into::<js_sys::JsString>() {
            let msg_div = document.create_element("div").unwrap();
            msg_div.set_inner_html(&txt.as_string().unwrap_or_default());
            messages_clone.append_child(&msg_div).unwrap();
        }
    });
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    // --- ปุ่มส่งข้อความ ---
    let ws_clone = ws.clone();
    let document = get_document();
    let send_button = document.get_element_by_id("send").unwrap();
    let input_box: HtmlInputElement = document
        .get_element_by_id("msg_input")
        .unwrap()
        .dyn_into()
        .unwrap();

    let uname = username.clone();
    let onclick = Closure::<dyn FnMut()>::new(move || {
        let text = input_box.value();
        if !text.is_empty() {
            let text = input_box.value();
            if !text.is_empty() {
                let full_msg = format!("<b>{}</b>: {}", uname, text);
                ws_clone.send_with_str(&full_msg).unwrap();
                input_box.set_value("");
            }
        }
    });
    send_button
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .set_onclick(Some(onclick.as_ref().unchecked_ref()));
    onclick.forget();

    Ok(())
}

fn get_document() -> Document {
    web_sys::window().unwrap().document().unwrap()
}

fn prompt(msg: &str) -> String {
    web_sys::window()
        .unwrap()
        .prompt_with_message(msg)
        .unwrap()
        .unwrap_or_else(|| "Anonymous".to_string())
}
