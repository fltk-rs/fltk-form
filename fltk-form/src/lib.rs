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
    ```rust,no_run
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

        while a.wait() {
            win.redraw();
        }
    }
    ```


    You can also rename properties using the rename_prop() method:
    ```rust,no_run
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

    #[allow(non_snake_case)]
    #[derive(Debug, Clone, FltkForm)]
    pub struct MyStruct {
        very_long_name: f64,
        second_value: f64,
        full_name: String,
        Choices: MyEnum,
        do_it: bool,
    }

    impl MyStruct {
        pub fn default() -> Self {
            Self {
                very_long_name: 0.0,
                second_value: 3.0,
                full_name: String::new(),
                Choices: MyEnum::A,
                do_it: true,
            }
        }
    }

    fn main() {
        let my_struct = MyStruct::default();

        let a = app::App::default().with_scheme(app::Scheme::Gtk);
        app::set_background_color(222, 222, 222);

        let mut win = window::Window::default().with_size(400, 300);
        let mut grp = group::Group::default()
            .with_size(300, 200)
            .center_of_parent()
            .with_label("Form 1")
            .with_align(enums::Align::Top | enums::Align::Left);

        let mut form = my_struct.generate();
        form.set_label("");
        form.rename_prop("very_long_name", "First Value");
        form.rename_prop("second_value", "Second Value");
        form.rename_prop("full_name", "Full name");
        form.rename_prop("do_it", "Do it?");

        grp.end();
        grp.set_frame(enums::FrameType::EngravedFrame);
        let mut btn = button::Button::default()
            .with_label("print")
            .with_size(80, 30)
            .below_of(&grp, 5)
            .center_x(&grp);
        win.end();
        win.show();

        let v = form.get_prop("First Value");
        assert_eq!(v, Some("0.0".to_owned()));

        btn.set_callback(move |_| {
            println!("{:?}", form.get_props());
        });

        a.run().unwrap();
    }
    ```
*/

use fltk::{image::*, prelude::*, utils::is_ptr_of, *};
use std::collections::HashMap;
use std::fmt;
use std::path::Path;

pub mod utils;

pub fn make_image_frame<P: AsRef<Path>>(filename: P) -> frame::Frame {
    let mut frame = frame::Frame::default();
    let img = SharedImage::load(&filename).ok();
    if let Some(ref img) = img {
        let w = img.width();
        let h = img.height();
        frame.set_size(w, h);
    }
    frame.set_image(img);
    frame.set_tooltip(filename.as_ref().to_str().unwrap());
    frame
}

#[derive(Debug, Clone)]
pub struct FlImage(pub String);

impl fmt::Display for FlImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug)]
#[non_exhaustive]
pub enum FltkFormError {
    FltkError(FltkErrorKind),
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
            FltkFormError::FltkError(ref err) => write!(f, "an fltk error occured {:?}", err),
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
                        let ptr = child.as_widget_ptr();
                        let val = if is_ptr_of::<input::Input>(ptr) {
                            1
                        } else if is_ptr_of::<button::CheckButton>(ptr) {
                            2
                        } else if is_ptr_of::<menu::Choice>(ptr) {
                            3
                        } else {
                            4
                        };
                        match val {
                            1 => {
                                let inp = input::Input::from_dyn_widget_ptr(ptr as _).unwrap();
                                return Some(inp.value());
                            }
                            2 => {
                                let inp =
                                    button::CheckButton::from_dyn_widget_ptr(ptr as _).unwrap();
                                return Some(inp.value().to_string());
                            }
                            3 => {
                                let choice = menu::Choice::from_dyn_widget_ptr(ptr as _).unwrap();
                                return choice.choice();
                            }
                            _ => {
                                let wid = widget::Widget::from_dyn_widget_ptr(ptr as _).unwrap();
                                return Some(wid.label());
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
                        let ptr = child.as_widget_ptr();
                        let val = if is_ptr_of::<input::Input>(ptr) {
                            1
                        } else if is_ptr_of::<button::CheckButton>(ptr) {
                            2
                        } else if is_ptr_of::<menu::Choice>(ptr) {
                            3
                        } else {
                            4
                        };
                        match val {
                            1 => {
                                let mut inp =
                                    input::Input::from_dyn_widget_ptr(ptr as _).unwrap();
                                inp.set_value(value);
                            }
                            2 => {
                                let mut inp =
                                    button::CheckButton::from_dyn_widget_ptr(ptr as _).unwrap();
                                let v = value == "true";
                                inp.set_value(v);
                            }
                            3 => {
                                let mut choice =
                                    menu::Choice::from_dyn_widget_ptr(ptr as _).unwrap();
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

    pub fn get_widget(&self, prop: &str) -> Option<Box<dyn WidgetExt>> {
        if let Some(grp) = self.as_group() {
            for child in grp.into_iter() {
                if child.label() == prop {
                    return Some(Box::new(child));
                }
            }
            None
        } else if self.label() == prop {
            let wid: widget::Widget = widget::Widget::from_dyn_widget_ptr(self.as_widget_ptr()).unwrap();
            Some(Box::new(wid))
        } else {
            None
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
        let i = make_image_frame(val.as_str());
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let val = format!("{}", *self);
        let i = make_image_frame(val.as_str());
        Box::new(i)
    }
}

impl FltkForm for f64 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::FloatInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
}

impl FltkForm for f32 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::FloatInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
}

impl FltkForm for i32 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
}

impl FltkForm for u32 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
}

impl FltkForm for i64 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
}

impl FltkForm for u64 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
}

impl FltkForm for isize {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
}

impl FltkForm for usize {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
}

impl FltkForm for i8 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
}

impl FltkForm for u8 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
}

impl FltkForm for i16 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
}

impl FltkForm for u16 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        Box::new(i)
    }
}

impl FltkForm for String {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::Input::default();
        i.set_value(self);
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default();
        i.set_value(self);
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
        Box::new(i)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut i = output::Output::default().with_align(enums::Align::Left);
        i.set_value(&format!("{}", *self));
        i.clear_visible_focus();
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
                let ptr = child.as_widget_ptr();
                let val = if is_ptr_of::<input::Input>(ptr) {
                    1
                } else if is_ptr_of::<button::CheckButton>(ptr) {
                    2
                } else if is_ptr_of::<menu::Choice>(ptr) {
                    3
                } else {
                    4
                };
                match val {
                    1 => {
                        let inp = input::Input::from_dyn_widget_ptr(ptr as _).unwrap();
                        return Some(inp.value());
                    }
                    2 => {
                        let inp = button::CheckButton::from_dyn_widget_ptr(ptr as _).unwrap();
                        return Some(format!("{}", inp.value()));
                    }
                    3 => {
                        let choice = menu::Choice::from_dyn_widget_ptr(ptr as _).unwrap();
                        return choice.choice();
                    }
                    _ => {
                        let wid = widget::Widget::from_dyn_widget_ptr(ptr as _).unwrap();
                        return Some(wid.label());
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
                let ptr = child.as_widget_ptr();
                let val = if is_ptr_of::<input::Input>(ptr) {
                    1
                } else if is_ptr_of::<button::CheckButton>(ptr) {
                    2
                } else if is_ptr_of::<menu::Choice>(ptr) {
                    3
                } else {
                    4
                };
                match val {
                    1 => {
                        let mut inp = input::Input::from_dyn_widget_ptr(ptr as _).unwrap();
                        inp.set_value(value);
                    }
                    2 => {
                        let mut inp = button::CheckButton::from_dyn_widget_ptr(ptr as _).unwrap();
                        let v = value == "true";
                        inp.set_value(v);
                    }
                    3 => {
                        let mut choice = menu::Choice::from_dyn_widget_ptr(ptr as _).unwrap();
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

#[allow(clippy::borrowed_box)]
fn get_widget_(wid: &Box<dyn WidgetExt>, prop: &str) -> Option<Box<dyn WidgetExt>> {
    if let Some(grp) = wid.as_group() {
        for child in grp.into_iter() {
            if child.label() == prop {
                return Some(Box::new(child));
            }
        }
        None
    } else if wid.label() == prop {
        let wid: widget::Widget = widget::Widget::from_dyn_widget_ptr(wid.as_widget_ptr()).unwrap();
        Some(Box::new(wid))
    } else {
        None
    }
}

pub trait HasProps {
    fn get_prop(&self, prop: &str) -> Option<String>;
    fn set_prop(&mut self, prop: &str, value: &str) -> Result<(), FltkFormError>;
    fn get_props(&self) -> HashMap<String, String>;
    fn rename_prop(&mut self, prop: &str, new_name: &str);
    fn get_widget(&self, prop: &str) -> Option<Box<dyn WidgetExt>>;
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
    fn get_widget(&self, prop: &str) -> Option<Box<dyn WidgetExt>> {
        get_widget_(self, prop)
    }
}
