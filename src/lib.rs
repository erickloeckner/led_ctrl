use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture, spawn_local};

use web_sys::{HtmlInputElement, HtmlSelectElement, Request, RequestInit, RequestMode, SvgElement};

struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

struct ColorHsv {
    h: f64,
    s: f64,
    v: f64,
}


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

fn get_svg(id: &str) -> SvgElement {
    document()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<SvgElement>()
        .unwrap()
}

fn get_input(id: &str) -> HtmlInputElement {
    document()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
}

fn get_pattern(id: &str) -> u8 {
    match document().get_element_by_id(id) {
        Some(v) => {
            match v.dyn_into::<HtmlSelectElement>() {
                Ok(v) => {
                    match u8::from_str_radix(&v.value(), 10) {
                        Ok(v) => v,
                        Err(_) => 0,
                    }
                }
                Err(_) => 0,
            }
        }
        None => 0,
    }
}

fn set_swatch(hue_id: &str, sat_id: &str, val_id: &str, swatch_id: &str) {
    let hue = get_value(hue_id);
    let sat = get_value(sat_id);
    let val = get_value(val_id);
    
    let swatch = get_svg(swatch_id);
    
    let rgb = hsv_2_rgb(&ColorHsv{ h: hue, s: sat, v: val });
    
    swatch.style().set_property("fill", &format!("rgba({}, {}, {}, 1.0)", rgb.r, rgb.g, rgb.b)).unwrap();
}

fn hsv_2_rgb(col: &ColorHsv) -> Pixel {
    let mut out = Pixel { r: 0, g: 0, b: 0 };
    let h_wrap = col.h.rem_euclid(1.0);
    
    match (h_wrap * 6.0).trunc() as u8 {
        0 => {
            out.r = (col.v * 255.0) as u8;
            out.g = ((col.v * (1.0 - col.s * (1.0 - ((col.h * 6.0) - ((col.h * 6.0).trunc()))))) * 255.0) as u8;
            out.b = ((col.v * (1.0 - col.s)) * 255.0) as u8;
        }
        1 => {
            out.r = ((col.v * (1.0 - col.s * ((col.h * 6.0) - ((col.h * 6.0).trunc())))) * 255.0) as u8;
            out.g = (col.v * 255.0) as u8;
            out.b = ((col.v * (1.0 - col.s)) * 255.0) as u8;
        }
        2 => {
            out.r = ((col.v * (1.0 - col.s)) * 255.0) as u8;
            out.g = (col.v * 255.0) as u8;
            out.b = ((col.v * (1.0 - col.s * (1.0 - ((col.h * 6.0) - ((col.h * 6.0).trunc()))))) * 255.0) as u8;
        }
        3 => {
            out.r = ((col.v * (1.0 - col.s)) * 255.0) as u8;
            out.g = ((col.v * (1.0 - col.s * ((col.h * 6.0) - ((col.h * 6.0).trunc())))) * 255.0) as u8;
            out.b = (col.v * 255.0) as u8;
        }
        4 => {
            out.r = ((col.v * (1.0 - col.s * (1.0 - ((col.h * 6.0) - ((col.h * 6.0).trunc()))))) * 255.0) as u8;
            out.g = ((col.v * (1.0 - col.s)) * 255.0) as u8;
            out.b = (col.v * 255.0) as u8;
        }
        5 => {
            out.r = (col.v * 255.0) as u8;
            out.g = ((col.v * (1.0 - col.s)) * 255.0) as u8;
            out.b = ((col.v * (1.0 - col.s * ((col.h * 6.0) - ((col.h * 6.0).trunc())))) * 255.0) as u8;
        }
        _ => (),
    }
    out
}

async fn set_leds() {
    {
        let url = "http://website.tld/led_api_01/set";
        
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
        
        let pattern = get_pattern("pattern");
        
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
            pattern,
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
    let color1_h = get_input("color1_h");
    let color1_h_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        set_swatch("color1_h", "color1_s", "color1_v", "color1_swatch");
    }) as Box<dyn FnMut(_)>);
    color1_h.add_event_listener_with_callback("change", color1_h_callback.as_ref().unchecked_ref())?;
    color1_h_callback.forget();
    
    let color1_s = get_input("color1_s");
    let color1_s_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        set_swatch("color1_h", "color1_s", "color1_v", "color1_swatch");
    }) as Box<dyn FnMut(_)>);
    color1_s.add_event_listener_with_callback("change", color1_s_callback.as_ref().unchecked_ref())?;
    color1_s_callback.forget();
    
    let color1_v = get_input("color1_v");
    let color1_v_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        set_swatch("color1_h", "color1_s", "color1_v", "color1_swatch");
    }) as Box<dyn FnMut(_)>);
    color1_v.add_event_listener_with_callback("change", color1_v_callback.as_ref().unchecked_ref())?;
    color1_v_callback.forget();
    
    let color2_h = get_input("color2_h");
    let color2_h_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        set_swatch("color2_h", "color2_s", "color2_v", "color2_swatch");
    }) as Box<dyn FnMut(_)>);
    color2_h.add_event_listener_with_callback("change", color2_h_callback.as_ref().unchecked_ref())?;
    color2_h_callback.forget();
    
    let color2_s = get_input("color2_s");
    let color2_s_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        set_swatch("color2_h", "color2_s", "color2_v", "color2_swatch");
    }) as Box<dyn FnMut(_)>);
    color2_s.add_event_listener_with_callback("change", color2_s_callback.as_ref().unchecked_ref())?;
    color2_s_callback.forget();
    
    let color2_v = get_input("color2_v");
    let color2_v_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        set_swatch("color2_h", "color2_s", "color2_v", "color2_swatch");
    }) as Box<dyn FnMut(_)>);
    color2_v.add_event_listener_with_callback("change", color2_v_callback.as_ref().unchecked_ref())?;
    color2_v_callback.forget();
    
    let color3_h = get_input("color3_h");
    let color3_h_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        set_swatch("color3_h", "color3_s", "color3_v", "color3_swatch");
    }) as Box<dyn FnMut(_)>);
    color3_h.add_event_listener_with_callback("change", color3_h_callback.as_ref().unchecked_ref())?;
    color3_h_callback.forget();
    
    let color3_s = get_input("color3_s");
    let color3_s_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        set_swatch("color3_h", "color3_s", "color3_v", "color3_swatch");
    }) as Box<dyn FnMut(_)>);
    color3_s.add_event_listener_with_callback("change", color3_s_callback.as_ref().unchecked_ref())?;
    color3_s_callback.forget();
    
    let color3_v = get_input("color3_v");
    let color3_v_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        set_swatch("color3_h", "color3_s", "color3_v", "color3_swatch");
    }) as Box<dyn FnMut(_)>);
    color3_v.add_event_listener_with_callback("change", color3_v_callback.as_ref().unchecked_ref())?;
    color3_v_callback.forget();
    
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
