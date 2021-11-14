/*!
    # fltk-form

    This crate aims to simplify generating gui from a data structure.

    ## Usage
    ```toml,no_run
    [dependencies]
    fltk = { git = "https://github.com/fltk-rs/fltk-rs" }
    fltk-form = { git = "https://github.com/MoAlyousef/fltk-form" }
    fltk-form-derive = { git = "https://github.com/MoAlyousef/fltk-form" }
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
        let s = MyStruct::new();

        let a = app::App::default().with_scheme(app::Scheme::Gtk);
        app::set_background_color(222, 222, 222);

        let mut win = window::Window::default().with_size(400, 300);
        let mut grp = group::Group::default().with_size(300, 200).center_of_parent();
        let w = s.generate();
        grp.end();
        grp.set_frame(enums::FrameType::EngravedFrame);
        win.end();
        win.show();

        let v = w.get_prop("b");
        assert_eq!(v, Some("3.0".to_owned()));

        a.run().unwrap();
    }
    ```
*/

use fltk::{prelude::*, *};
use std::os::raw;

pub trait FltkForm {
    fn generate(&self) -> Box<dyn WidgetExt>;
}

impl FltkForm for f64 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::FloatInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(Box::into_raw(Box::new(val)) as *mut raw::c_void);
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
            i.set_raw_user_data(Box::into_raw(Box::new(val)) as *mut raw::c_void);
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
            i.set_raw_user_data(Box::into_raw(Box::new(val)) as *mut raw::c_void);
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
            i.set_raw_user_data(Box::into_raw(Box::new(val)) as *mut raw::c_void);
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
            i.set_raw_user_data(Box::into_raw(Box::new(val)) as *mut raw::c_void);
        }
        Box::new(i)
    }
}

impl FltkForm for bool {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = button::CheckButton::default().with_align(enums::Align::Left);
        let val = format!("{:?}", *self);
        i.set_value(*self);
        i.clear_visible_focus();
        unsafe {
            i.set_raw_user_data(Box::into_raw(Box::new(val)) as *mut raw::c_void);
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
                ptr as *const _ as *mut String
            };
            unsafe {
                return Some((*val).clone());
            }
        }
    }
    None
}

#[allow(clippy::borrowed_box)]
fn get_props_(wid: &Box<dyn WidgetExt>) -> Vec<(String, String)> {
    let wid = unsafe { wid.as_widget_ptr() };
    let grp = unsafe { group::Group::from_widget_ptr(wid as _) };
    let mut temp = vec![];
    for child in grp.into_iter() {
        if !child.label().is_empty() && unsafe { !child.raw_user_data().is_null() } {
            unsafe {
                temp.push((
                    child.label().clone(),
                    (*(child.raw_user_data() as *const _ as *mut String)).clone(),
                ));
            }
        }
    }
    temp
}

pub trait HasProps {
    fn get_prop(&self, prop: &str) -> Option<String>;
    fn get_props(&self) -> Vec<(String, String)>;
}

impl HasProps for Box<dyn WidgetExt> {
    fn get_prop(&self, prop: &str) -> Option<String> {
        get_prop_(self, prop)
    }
    fn get_props(&self) -> Vec<(String, String)> {
        get_props_(self)
    }
}
