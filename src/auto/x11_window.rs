// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gdk_x11_sys;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct X11Window(Object<gdk_x11_sys::GdkX11Window, gdk_x11_sys::GdkX11WindowClass, X11WindowClass>) @extends gdk::Window;

    match fn {
        get_type => || gdk_x11_sys::gdk_x11_window_get_type(),
    }
}

impl X11Window {
    //pub fn foreign_new_for_display(display: &X11Display, window: /*Ignored*/xlib::Window) -> X11Window {
    //    unsafe { TODO: call gdk_x11_sys:gdk_x11_window_foreign_new_for_display() }
    //}

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub fn get_desktop(&self) -> u32 {
        unsafe { gdk_x11_sys::gdk_x11_window_get_desktop(self.to_glib_none().0) }
    }

    //pub fn get_xid(&self) -> /*Ignored*/xlib::Window {
    //    unsafe { TODO: call gdk_x11_sys:gdk_x11_window_get_xid() }
    //}

    pub fn move_to_current_desktop(&self) {
        unsafe {
            gdk_x11_sys::gdk_x11_window_move_to_current_desktop(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub fn move_to_desktop(&self, desktop: u32) {
        unsafe {
            gdk_x11_sys::gdk_x11_window_move_to_desktop(self.to_glib_none().0, desktop);
        }
    }

    #[cfg_attr(feature = "v3_12", deprecated)]
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub fn set_frame_extents(&self, left: i32, right: i32, top: i32, bottom: i32) {
        unsafe {
            gdk_x11_sys::gdk_x11_window_set_frame_extents(
                self.to_glib_none().0,
                left,
                right,
                top,
                bottom,
            );
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    pub fn set_frame_sync_enabled(&self, frame_sync_enabled: bool) {
        unsafe {
            gdk_x11_sys::gdk_x11_window_set_frame_sync_enabled(
                self.to_glib_none().0,
                frame_sync_enabled.to_glib(),
            );
        }
    }

    #[cfg(any(feature = "v3_4", feature = "dox"))]
    pub fn set_hide_titlebar_when_maximized(&self, hide_titlebar_when_maximized: bool) {
        unsafe {
            gdk_x11_sys::gdk_x11_window_set_hide_titlebar_when_maximized(
                self.to_glib_none().0,
                hide_titlebar_when_maximized.to_glib(),
            );
        }
    }

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    pub fn set_theme_variant(&self, variant: &str) {
        unsafe {
            gdk_x11_sys::gdk_x11_window_set_theme_variant(
                self.to_glib_none().0,
                variant.to_glib_none().0,
            );
        }
    }

    pub fn set_user_time(&self, timestamp: u32) {
        unsafe {
            gdk_x11_sys::gdk_x11_window_set_user_time(self.to_glib_none().0, timestamp);
        }
    }

    #[cfg(any(feature = "v3_4", feature = "dox"))]
    pub fn set_utf8_property(&self, name: &str, value: Option<&str>) {
        unsafe {
            gdk_x11_sys::gdk_x11_window_set_utf8_property(
                self.to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    //pub fn lookup_for_display(display: &X11Display, window: /*Ignored*/xlib::Window) -> Option<X11Window> {
    //    unsafe { TODO: call gdk_x11_sys:gdk_x11_window_lookup_for_display() }
    //}
}

impl fmt::Display for X11Window {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11Window")
    }
}
