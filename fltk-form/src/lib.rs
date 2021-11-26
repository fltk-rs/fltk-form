/*!
    # fltk-form

    This crate aims to simplify generating gui from a data structure.

    ## Usage
    ```toml,no_run
    [dependencies]
    fltk = "1.2.16"
    fltk-form = "0.1"
    fltk-form-derive = "0.1"
    ```

    ## Example
    ```rust
    #[macro_use]
    extern crate fltk_form_derive;

    use fltk::{prelude::*, *};
    use fltk_form::{FltkForm, HasProps, FlImage};

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
        f:FlImage,
    }

    impl MyStruct {
        pub fn default() -> Self {
            Self {
                a: 0.0,
                b: 3.0,
                c: String::new(),
                d: MyEnum::A,
                e: true,
                f:FlImage(String::from("examples/orange_circle.svg")),
            }
        }
    }

    fn main() {
        let my_struct = MyStruct::default();

        let a = app::App::default().with_scheme(app::Scheme::Gtk);
        app::set_background_color(222, 222, 222);

        let mut win = window::Window::default().with_size(400, 300);
        let mut grp = group::Scroll::default()
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

use fltk::{prelude::*, image::*, *};
use std::collections::HashMap;
use std::fmt;
use std::fs;

use std::mem::transmute;

pub fn make_image(filename:&str) -> Option<SharedImage> {
    let mut image_filename:String = match fs::canonicalize(filename) {
        Ok(image_filename) => image_filename.to_str().unwrap().to_owned(),
        Err(e) => {
            println!("ERROR: {:?}\nFilename:{:?}",e, filename);
            return None
        },
    };
    println!("image:{:?}",image_filename);
    let mut image = SharedImage::load(image_filename.as_str());
    if image.is_ok() {
        let current_image = image.ok().unwrap();
        return Some(current_image)
    }
    None
}
pub fn make_frame(filename:&str) -> frame::Frame {
    let mut frame = frame::Frame::default();
    let img = make_image(filename.clone());
    frame.set_image(img.clone());
    if let Some(img) = img {
        let w = img.width();
        let h = img.height();
        frame.set_size(w,h);
        println!("some img");
    }
    frame.clone()
}
#[derive(Debug, Clone)]
pub struct FlImage(pub String);
impl fmt::Display for FlImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match &*self {
            FlImage(e) => v = e.clone(),
        }
        write!(f, "{}", v.as_str())
    }
}
#[derive(Debug)]
#[non_exhaustive]
pub enum FltkFormError {
    Internal(FltkFormErrorKind),
    Unknown(String),
}

unsafe impl Send for FltkFormError {}
unsafe impl Sync for FltkFormError {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FltkFormErrorKind {
    PropertyInexistent,
    FailedToChangeData,
}

impl std::error::Error for FltkFormError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl fmt::Display for FltkFormError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FltkFormError::Internal(ref err) => write!(f, "An internal error occured {:?}", err),
            FltkFormError::Unknown(ref err) => write!(f, "An unknown error occurred {:?}", err),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Form {
    grp: group::Group,
}

impl Default for Form {
    fn default() -> Self {
        Form::new(0, 0, 0, 0, None)
    }
}

impl Form {
    pub fn new<S: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, label: S) -> Self {
        let grp = group::Group::new(x, y, w, h, label);
        grp.end();
        Self { grp }
    }

    pub fn default_fill() -> Self {
        Form::default().size_of_parent().center_of_parent()
    }

    pub fn set_data<T: FltkForm>(&mut self, data: T) {
        self.begin();
        let mut w = data.generate();
        w.resize(self.x(), self.y(), self.w(), self.h());
        self.end();
    }

    pub fn from_data<T: FltkForm>(mut self, data: T) -> Self {
        self.set_data(data);
        self
    }

    pub fn set_data_view<T: FltkForm>(&mut self, data: T) {
        self.begin();
        let mut w = data.view();
        w.resize(self.x(), self.y(), self.w(), self.h());
        self.end();
    }

    pub fn from_data_view<T: FltkForm>(mut self, data: T) -> Self {
        self.set_data_view(data);
        self
    }

    pub fn get_prop(&self, prop: &str) -> Option<String> {
        if let Some(child) = self.grp.child(0) {
            if let Some(grp) = child.as_group() {
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
                                let inp = unsafe {
                                    input::Input::from_widget_ptr(child.as_widget_ptr() as _)
                                };
                                return Some(inp.value());
                            }
                            2 => {
                                let inp = unsafe {
                                    button::CheckButton::from_widget_ptr(child.as_widget_ptr() as _)
                                };
                                return Some(format!("{}", inp.value()));
                            }
                            3 => {
                                let choice = unsafe {
                                    menu::Choice::from_widget_ptr(child.as_widget_ptr() as _)
                                };
                                return choice.choice();
                            }
                            _ => {
                                return None;
                            }
                        }
                    }
                }
                None
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn set_prop(&mut self, prop: &str, value: &str) -> Result<(), FltkFormError> {
        let mut found = false;
        if let Some(child) = self.grp.child(0) {
            if let Some(grp) = child.as_group() {
                for child in grp.into_iter() {
                    if child.label() == prop {
                        found = true;
                        let val = unsafe {
                            let ptr = child.raw_user_data();
                            if ptr.is_null() {
                                return Err(FltkFormError::Internal(
                                    FltkFormErrorKind::FailedToChangeData,
                                ));
                            }
                            ptr as usize
                        };
                        match val {
                            1 => {
                                let mut inp = unsafe {
                                    input::Input::from_widget_ptr(child.as_widget_ptr() as _)
                                };
                                inp.set_value(value);
                            }
                            2 => {
                                let mut inp = unsafe {
                                    button::CheckButton::from_widget_ptr(child.as_widget_ptr() as _)
                                };
                                let v = value == "true";
                                inp.set_value(v);
                            }
                            3 => {
                                let mut choice = unsafe {
                                    menu::Choice::from_widget_ptr(child.as_widget_ptr() as _)
                                };
                                let idx = choice.find_index(value);
                                choice.set_value(idx);
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
        if !found {
            return Err(FltkFormError::Internal(
                FltkFormErrorKind::PropertyInexistent,
            ));
        }
        Ok(())
    }

    pub fn get_props(&self) -> HashMap<String, String> {
        let mut temp = HashMap::new();
        if let Some(c) = self.grp.child(0) {
            if let Some(grp) = c.as_group() {
                for child in grp.into_iter() {
                    if !child.label().is_empty() {
                        if let Some(prop) = self.get_prop(&child.label()) {
                            temp.insert(child.label().clone(), prop);
                        }
                    }
                }
            }
        }
        temp
    }

    pub fn rename_prop(&self, prop: &str, new_name: &str) {
        if let Some(child) = self.grp.child(0) {
            if let Some(grp) = child.as_group() {
                for mut child in grp.into_iter() {
                    if child.label() == prop {
                        child.set_label(new_name);
                    }
                }
            }
        }
    }
}

fltk::widget_extends!(Form, group::Group, grp);

pub trait FltkForm {
    fn generate(&self) -> Box<dyn WidgetExt>;
    fn view(&self) -> Box<dyn WidgetExt>;
}
impl FltkForm for FlImage {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let val = format!("{}", *self);
        let mut i = make_frame(val.as_str());
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let val = format!("{}", *self);
        let mut i = make_frame(val.as_str());
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
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
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
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
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
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
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for u32 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
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
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for u64 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for isize {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for usize {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for i8 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for u8 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for i16 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for u16 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
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
        i.set_value(self);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        i.set_value(self);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for &str {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let i = frame::Frame::default().with_label(self);
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let i = frame::Frame::default().with_label(self);
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
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default().with_align(enums::Align::Left);
        i.set_value(&format!("{}", *self));
        i.clear_visible_focus();
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl<T> FltkForm for Vec<T>
where
    T: FltkForm,
{
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut g = group::Pack::default();
        g.set_spacing(5);
        for v in self.iter() {
            let mut w = v.generate();
            w.set_align(enums::Align::Left);
            w.set_size(w.w(), 30);
        }
        g.end();
        Box::new(g)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut g = group::Pack::default();
        g.set_spacing(5);
        for v in self.iter() {
            let mut w = v.view();
            w.set_align(enums::Align::Left);
            w.set_size(w.w(), 30);
        }
        g.end();
        Box::new(g)
    }
}

#[allow(clippy::borrowed_box)]
fn rename_prop_(wid: &Box<dyn WidgetExt>, prop: &str, new_name: &str) {
    if let Some(grp) = wid.as_group() {
        for mut child in grp.into_iter() {
            if child.label() == prop {
                child.set_label(new_name);
            }
        }
    }
}

#[allow(clippy::borrowed_box)]
fn get_prop_(wid: &Box<dyn WidgetExt>, prop: &str) -> Option<String> {
    if let Some(grp) = wid.as_group() {
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
                        let inp =
                            unsafe { input::Input::from_widget_ptr(child.as_widget_ptr() as _) };
                        return Some(inp.value());
                    }
                    2 => {
                        let inp = unsafe {
                            button::CheckButton::from_widget_ptr(child.as_widget_ptr() as _)
                        };
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
    } else {
        None
    }
}

#[allow(clippy::borrowed_box)]
fn set_prop_(wid: &Box<dyn WidgetExt>, prop: &str, value: &str) -> Result<(), FltkFormError> {
    let mut found = false;
    if let Some(grp) = wid.as_group() {
        for child in grp.into_iter() {
            if child.label() == prop {
                found = true;
                let val = unsafe {
                    let ptr = child.raw_user_data();
                    if ptr.is_null() {
                        return Err(FltkFormError::Internal(
                            FltkFormErrorKind::FailedToChangeData,
                        ));
                    }
                    ptr as usize
                };
                match val {
                    1 => {
                        let mut inp =
                            unsafe { input::Input::from_widget_ptr(child.as_widget_ptr() as _) };
                        inp.set_value(value);
                    }
                    2 => {
                        let mut inp = unsafe {
                            button::CheckButton::from_widget_ptr(child.as_widget_ptr() as _)
                        };
                        let v = value == "true";
                        inp.set_value(v);
                    }
                    3 => {
                        let mut choice =
                            unsafe { menu::Choice::from_widget_ptr(child.as_widget_ptr() as _) };
                        let idx = choice.find_index(value);
                        choice.set_value(idx);
                    }
                    _ => (),
                }
            }
        }
    }
    if !found {
        return Err(FltkFormError::Internal(
            FltkFormErrorKind::PropertyInexistent,
        ));
    }
    Ok(())
}

#[allow(clippy::borrowed_box)]
fn get_props_(wid: &Box<dyn WidgetExt>) -> HashMap<String, String> {
    let mut temp = HashMap::new();
    if let Some(grp) = wid.as_group() {
        for child in grp.into_iter() {
            if !child.label().is_empty() {
                if let Some(prop) = get_prop_(wid, &child.label()) {
                    temp.insert(child.label().clone(), prop);
                }
            }
        }
    }
    temp
}

pub trait HasProps {
    fn get_prop(&self, prop: &str) -> Option<String>;
    fn set_prop(&mut self, prop: &str, value: &str) -> Result<(), FltkFormError>;
    fn get_props(&self) -> HashMap<String, String>;
    fn rename_prop(&mut self, prop: &str, new_name: &str);
}

impl HasProps for Box<dyn WidgetExt> {
    fn get_prop(&self, prop: &str) -> Option<String> {
        get_prop_(self, prop)
    }
    fn set_prop(&mut self, prop: &str, value: &str) -> Result<(), FltkFormError> {
        set_prop_(self, prop, value)
    }
    fn get_props(&self) -> HashMap<String, String> {
        get_props_(self)
    }
    fn rename_prop(&mut self, prop: &str, new_name: &str) {
        rename_prop_(self, prop, new_name);
    }
}
