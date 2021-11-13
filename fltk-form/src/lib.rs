use fltk::{prelude::*, *};
use fltk_sys::widget;
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
            app::set_raw_callback(
                &mut i,
                Box::into_raw(Box::new(val)) as *mut raw::c_void,
                None,
            );
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
            app::set_raw_callback(
                &mut i,
                Box::into_raw(Box::new(val)) as *mut raw::c_void,
                None,
            );
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
            app::set_raw_callback(
                &mut i,
                Box::into_raw(Box::new(val)) as *mut raw::c_void,
                None,
            );
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
            app::set_raw_callback(
                &mut i,
                Box::into_raw(Box::new(val)) as *mut raw::c_void,
                None,
            );
        }
        Box::new(i)
    }
}

impl FltkForm for String {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::Input::default();
        let val = self.clone();
        i.set_value(&val.clone());
        unsafe {
            app::set_raw_callback(
                &mut i,
                Box::into_raw(Box::new(val)) as *mut raw::c_void,
                None,
            );
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
            app::set_raw_callback(
                &mut i,
                Box::into_raw(Box::new(val)) as *mut raw::c_void,
                None,
            );
        }
        Box::new(i)
    }
}

fn get_prop_(wid: &Box<dyn WidgetExt>, prop: &str) -> Option<String> {
    let wid = unsafe { wid.as_widget_ptr() };
    let grp = unsafe { group::Group::from_widget_ptr(wid as _) };
    for child in grp.into_iter() {
        if child.label() == prop {
            let val =
                unsafe {
                    Box::from_raw(widget::Fl_Widget_user_data(child.as_widget_ptr() as _)
                        as *const _ as *mut String)
                };
            return Some(*val);
        }
    }
    None
}

pub trait HasProps {
    fn get_prop(&self, prop: &str) -> Option<String>;
}

impl HasProps for Box<dyn WidgetExt> {
    fn get_prop(&self, prop: &str) -> Option<String> {
        get_prop_(self, prop)
    }
}