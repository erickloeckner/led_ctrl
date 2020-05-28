use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture, spawn_local};

use web_sys::{HtmlInputElement, Request, RequestInit, RequestMode};

fn window() -> web_sys::Window {
    web_sys::window().unwrap()
}

fn document() -> web_sys::Document {
    window().document().unwrap()
}

fn get_value(id: &str) -> f64 {
    match document().get_element_by_id(id) {
        Some(v) => {
            match v.dyn_into::<HtmlInputElement>() {
                Ok(v) => v.value_as_number(),
                Err(_) => 0.0,
            }
        }
        None => 0.0,
    }
}

async fn set_leds() {
    {
        let url = "http://home.erickloeckner.com/led_api_01/set";
        
        let mut req_opts = RequestInit::new();
        req_opts.method("POST");
        req_opts.mode(RequestMode::Cors);
        
        let color1_h = get_value("color1_h");
        let color1_s = get_value("color1_s");
        let color1_v = get_value("color1_v");
        
        let color2_h = get_value("color2_h");
        let color2_s = get_value("color2_s");
        let color2_v = get_value("color2_v");
        
        let color3_h = get_value("color3_h");
        let color3_s = get_value("color3_s");
        let color3_v = get_value("color3_v");
        
        //~ let color1_h = document()
            //~ .get_element_by_id("color1_h")
            //~ .unwrap()
            //~ .dyn_into::<HtmlInputElement>()
            //~ .unwrap();
        //~ let color1_s = document()
            //~ .get_element_by_id("color1_s")
            //~ .unwrap()
            //~ .dyn_into::<HtmlInputElement>()
            //~ .unwrap();
        //~ let color1_v = document()
            //~ .get_element_by_id("color1_v")
            //~ .unwrap()
            //~ .dyn_into::<HtmlInputElement>()
            //~ .unwrap();
            
        //~ let color2_h = document()
            //~ .get_element_by_id("color2_h")
            //~ .unwrap()
            //~ .dyn_into::<HtmlInputElement>()
            //~ .unwrap();
        //~ let color2_s = document()
            //~ .get_element_by_id("color2_s")
            //~ .unwrap()
            //~ .dyn_into::<HtmlInputElement>()
            //~ .unwrap();
        //~ let color2_v = document()
            //~ .get_element_by_id("color2_v")
            //~ .unwrap()
            //~ .dyn_into::<HtmlInputElement>()
            //~ .unwrap();
            
        //~ let color3_h = document()
            //~ .get_element_by_id("color3_h")
            //~ .unwrap()
            //~ .dyn_into::<HtmlInputElement>()
            //~ .unwrap();
        //~ let color3_s = document()
            //~ .get_element_by_id("color3_s")
            //~ .unwrap()
            //~ .dyn_into::<HtmlInputElement>()
            //~ .unwrap();
        //~ let color3_v = document()
            //~ .get_element_by_id("color3_v")
            //~ .unwrap()
            //~ .dyn_into::<HtmlInputElement>()
            //~ .unwrap();
        
        //~ let body_str = "color1_h={}&color1_s={}&color1_v={}&color2_h={}&color2_s={}&color2_v={}&color3_h={}&color3_s={}&color3_v={}&pattern={}";
        let body = format!(
            "color1_h={}&color1_s={}&color1_v={}&color2_h={}&color2_s={}&color2_v={}&color3_h={}&color3_s={}&color3_v={}&pattern={}",
            color1_h,
            color1_s,
            color1_v,
            color2_h,
            color2_s,
            color2_v,
            color3_h,
            color3_s,
            color3_v,
            1,
        );
        req_opts.body(Some(&JsValue::from_str(&body)));
        
        let request = Request::new_with_str_and_init(url, &req_opts).unwrap();
        request.headers().set("Content-Type", "application/x-www-form-urlencoded").unwrap();
        
        let resp_value = JsFuture::from(window().fetch_with_request(&request)).await;
        match resp_value {
            Ok(_v)  => (),
            Err(_e) => (),
        }
    }
}

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    let send_btn = document()
        .get_element_by_id("send")
        .unwrap();
    let send_btn_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        {
            spawn_local(async {
                set_leds().await;
            });
        }
    }) as Box<dyn FnMut(_)>);
    send_btn.add_event_listener_with_callback("mousedown", send_btn_callback.as_ref().unchecked_ref())?;
    send_btn_callback.forget();
    
    Ok(())
}
