/*!
    # fltk-form

    This crate aims to simplify generating gui from a data structure.

    ## Usage
    ```toml,no_run
    [dependencies]
    fltk = 1.2.16
    fltk-form = 0.1
    fltk-form-derive = 0.1
    ```

    ## Example
    ```rust
    #[macro_use]
    extern crate fltk_form_derive;

    use fltk::{prelude::*, *};
    use fltk_form::{FltkForm, HasProps};

    #[derive(Copy, Debug, Clone, FltkForm)]
    pub enum MyEnum {
        A,
        B,
        C,
    }

    #[derive(Debug, Clone, FltkForm)]
    pub struct MyStruct {
        a: f64,
        b: f64,
        c: String,
        d: MyEnum,
        e: bool,
    }

    impl MyStruct {
        pub fn new() -> Self {
            Self {
                a: 0.0,
                b: 3.0,
                c: String::new(),
                d: MyEnum::A,
                e: true,
            }
        }
    }

    fn main() {
        let my_struct = MyStruct::new();

        let a = app::App::default().with_scheme(app::Scheme::Gtk);
        app::set_background_color(222, 222, 222);

        let mut win = window::Window::default().with_size(400, 300);
        let mut grp = group::Group::default()
            .with_size(300, 200)
            .center_of_parent();
        let form = my_struct.generate();
        grp.end();
        let mut btn = button::Button::default()
            .with_label("print")
            .with_size(80, 30)
            .below_of(&grp, 5)
            .center_x(&grp);
        grp.set_frame(enums::FrameType::EngravedFrame);
        win.end();
        win.show();

        let v = form.get_prop("b");
        assert_eq!(v, Some("3.0".to_owned()));

        btn.set_callback(move |_| {
            println!("{:?}", form.get_props());
        });

        a.run().unwrap();
    }
    ```
*/

use fltk::{prelude::*, *};
use std::collections::HashMap;
use std::mem::transmute;

pub trait FltkForm {
    fn generate(&self) -> Box<dyn WidgetExt>;
}

impl FltkForm for f64 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::FloatInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for f32 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::FloatInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for i32 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for i64 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for String {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::Input::default();
        let val = self.clone();
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for bool {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = button::CheckButton::default().with_align(enums::Align::Left);
        i.set_value(*self);
        i.clear_visible_focus();
        unsafe {
            i.set_raw_user_data(transmute(2_usize));
        }
        Box::new(i)
    }
}

#[allow(clippy::borrowed_box)]
fn get_prop_(wid: &Box<dyn WidgetExt>, prop: &str) -> Option<String> {
    let wid = unsafe { wid.as_widget_ptr() };
    let grp = unsafe { group::Group::from_widget_ptr(wid as _) };
    for child in grp.into_iter() {
        if child.label() == prop {
            let val = unsafe {
                let ptr = child.raw_user_data();
                if ptr.is_null() {
                    return None;
                }
                ptr as usize
            };
            match val {
                1 => {
                    let inp = unsafe { input::Input::from_widget_ptr(child.as_widget_ptr() as _) };
                    return Some(inp.value());
                }
                2 => {
                    let inp =
                        unsafe { button::CheckButton::from_widget_ptr(child.as_widget_ptr() as _) };
                    return Some(format!("{}", inp.value()));
                }
                3 => {
                    let choice =
                        unsafe { menu::Choice::from_widget_ptr(child.as_widget_ptr() as _) };
                    return choice.choice();
                }
                _ => {
                    return None;
                }
            }
        }
    }
    None
}

#[allow(clippy::borrowed_box)]
fn get_props_(wid: &Box<dyn WidgetExt>) -> HashMap<String, String> {
    let w = unsafe { wid.as_widget_ptr() };
    let grp = unsafe { group::Group::from_widget_ptr(w as _) };
    let mut temp = HashMap::new();
    for child in grp.into_iter() {
        if !child.label().is_empty() {
            temp.insert(
                child.label().clone(),
                get_prop_(wid, &child.label()).unwrap(),
            );
        }
    }
    temp
}

pub trait HasProps {
    fn get_prop(&self, prop: &str) -> Option<String>;
    fn get_props(&self) -> HashMap<String, String>;
}

impl HasProps for Box<dyn WidgetExt> {
    fn get_prop(&self, prop: &str) -> Option<String> {
        get_prop_(self, prop)
    }
    fn get_props(&self) -> HashMap<String, String> {
        get_props_(self)
    }
}
