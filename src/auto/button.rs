// This file was generated by gir (0fe730d) from gir-files (db49619)
// DO NOT EDIT

use Actionable;
use Bin;
use Container;
use PositionType;
use ReliefStyle;
use Widget;
use ffi;
use gdk;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Button(Object<ffi::GtkButton>): Bin, Container, Widget, Actionable;

    match fn {
        get_type => || ffi::gtk_button_get_type(),
    }
}

impl Button {
    pub fn new() -> Button {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_new()).downcast_unchecked()
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub fn new_from_icon_name<'a, P: Into<Option<&'a str>>>(icon_name: P, size: i32) -> Button {
        assert_initialized_main_thread!();
        let icon_name = icon_name.into();
        let icon_name = icon_name.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_new_from_icon_name(icon_name.0, size)).downcast_unchecked()
        }
    }

    pub fn new_from_stock(stock_id: &str) -> Button {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_new_from_stock(stock_id.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_label(label: &str) -> Button {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_new_with_label(label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> Button {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_new_with_mnemonic(label.to_glib_none().0)).downcast_unchecked()
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ButtonExt {
    fn clicked(&self);

    fn get_alignment(&self) -> (f32, f32);

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_always_show_image(&self) -> bool;

    fn get_event_window(&self) -> Option<gdk::Window>;

    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn get_focus_on_click(&self) -> bool;

    fn get_image(&self) -> Option<Widget>;

    fn get_image_position(&self) -> PositionType;

    fn get_label(&self) -> Option<String>;

    fn get_relief(&self) -> ReliefStyle;

    fn get_use_stock(&self) -> bool;

    fn get_use_underline(&self) -> bool;

    fn set_alignment(&self, xalign: f32, yalign: f32);

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_always_show_image(&self, always_show: bool);

    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn set_focus_on_click(&self, focus_on_click: bool);

    fn set_image<P: IsA<Widget>>(&self, image: &P);

    fn set_image_position(&self, position: PositionType);

    fn set_label(&self, label: &str);

    fn set_relief(&self, relief: ReliefStyle);

    fn set_use_stock(&self, use_stock: bool);

    fn set_use_underline(&self, use_underline: bool);

    fn get_property_xalign(&self) -> f32;

    fn set_property_xalign(&self, xalign: f32);

    fn get_property_yalign(&self) -> f32;

    fn set_property_yalign(&self, yalign: f32);

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate(&self);

    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_clicked(&self);

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_always_show_image_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_image_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_image_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_relief_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_stock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Button> + IsA<glib::object::Object> + glib::object::ObjectExt> ButtonExt for O {
    fn clicked(&self) {
        unsafe {
            ffi::gtk_button_clicked(self.to_glib_none().0);
        }
    }

    fn get_alignment(&self) -> (f32, f32) {
        unsafe {
            let mut xalign = mem::uninitialized();
            let mut yalign = mem::uninitialized();
            ffi::gtk_button_get_alignment(self.to_glib_none().0, &mut xalign, &mut yalign);
            (xalign, yalign)
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_always_show_image(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_get_always_show_image(self.to_glib_none().0))
        }
    }

    fn get_event_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_button_get_event_window(self.to_glib_none().0))
        }
    }

    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn get_focus_on_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_get_focus_on_click(self.to_glib_none().0))
        }
    }

    fn get_image(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_button_get_image(self.to_glib_none().0))
        }
    }

    fn get_image_position(&self) -> PositionType {
        unsafe {
            from_glib(ffi::gtk_button_get_image_position(self.to_glib_none().0))
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_button_get_label(self.to_glib_none().0))
        }
    }

    fn get_relief(&self) -> ReliefStyle {
        unsafe {
            from_glib(ffi::gtk_button_get_relief(self.to_glib_none().0))
        }
    }

    fn get_use_stock(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_get_use_stock(self.to_glib_none().0))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_get_use_underline(self.to_glib_none().0))
        }
    }

    fn set_alignment(&self, xalign: f32, yalign: f32) {
        unsafe {
            ffi::gtk_button_set_alignment(self.to_glib_none().0, xalign, yalign);
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_always_show_image(&self, always_show: bool) {
        unsafe {
            ffi::gtk_button_set_always_show_image(self.to_glib_none().0, always_show.to_glib());
        }
    }

    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn set_focus_on_click(&self, focus_on_click: bool) {
        unsafe {
            ffi::gtk_button_set_focus_on_click(self.to_glib_none().0, focus_on_click.to_glib());
        }
    }

    fn set_image<P: IsA<Widget>>(&self, image: &P) {
        unsafe {
            ffi::gtk_button_set_image(self.to_glib_none().0, image.to_glib_none().0);
        }
    }

    fn set_image_position(&self, position: PositionType) {
        unsafe {
            ffi::gtk_button_set_image_position(self.to_glib_none().0, position.to_glib());
        }
    }

    fn set_label(&self, label: &str) {
        unsafe {
            ffi::gtk_button_set_label(self.to_glib_none().0, label.to_glib_none().0);
        }
    }

    fn set_relief(&self, relief: ReliefStyle) {
        unsafe {
            ffi::gtk_button_set_relief(self.to_glib_none().0, relief.to_glib());
        }
    }

    fn set_use_stock(&self, use_stock: bool) {
        unsafe {
            ffi::gtk_button_set_use_stock(self.to_glib_none().0, use_stock.to_glib());
        }
    }

    fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            ffi::gtk_button_set_use_underline(self.to_glib_none().0, use_underline.to_glib());
        }
    }

    fn get_property_xalign(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "xalign".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_xalign(&self, xalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "xalign".to_glib_none().0, Value::from(&xalign).to_glib_none().0);
        }
    }

    fn get_property_yalign(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "yalign".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_yalign(&self, yalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "yalign".to_glib_none().0, Value::from(&yalign).to_glib_none().0);
        }
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate",
                transmute(activate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_activate(&self) {
        let _ = self.emit("activate", &[]).unwrap();
    }

    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "clicked",
                transmute(clicked_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_clicked(&self) {
        let _ = self.emit("clicked", &[]).unwrap();
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_always_show_image_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::always-show-image",
                transmute(notify_always_show_image_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_image_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::image",
                transmute(notify_image_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_image_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::image-position",
                transmute(notify_image_position_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::label",
                transmute(notify_label_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_relief_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::relief",
                transmute(notify_relief_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_use_stock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-stock",
                transmute(notify_use_stock_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-underline",
                transmute(notify_use_underline_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::xalign",
                transmute(notify_xalign_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::yalign",
                transmute(notify_yalign_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_trampoline<P>(this: *mut ffi::GtkButton, f: glib_ffi::gpointer)
where P: IsA<Button> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Button::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn clicked_trampoline<P>(this: *mut ffi::GtkButton, f: glib_ffi::gpointer)
where P: IsA<Button> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Button::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_6", feature = "dox"))]
unsafe extern "C" fn notify_always_show_image_trampoline<P>(this: *mut ffi::GtkButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Button> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Button::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_image_trampoline<P>(this: *mut ffi::GtkButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Button> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Button::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_image_position_trampoline<P>(this: *mut ffi::GtkButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Button> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Button::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_label_trampoline<P>(this: *mut ffi::GtkButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Button> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Button::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_relief_trampoline<P>(this: *mut ffi::GtkButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Button> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Button::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_use_stock_trampoline<P>(this: *mut ffi::GtkButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Button> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Button::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_use_underline_trampoline<P>(this: *mut ffi::GtkButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Button> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Button::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_xalign_trampoline<P>(this: *mut ffi::GtkButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Button> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Button::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_yalign_trampoline<P>(this: *mut ffi::GtkButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Button> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Button::from_glib_borrow(this).downcast_unchecked())
}
