// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gdk_x11_sys;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use xlib;
use X11Visual;

glib_wrapper! {
    pub struct X11Screen(Object<gdk_x11_sys::GdkX11Screen, gdk_x11_sys::GdkX11ScreenClass, X11ScreenClass>) @extends gdk::Screen;

    match fn {
        get_type => || gdk_x11_sys::gdk_x11_screen_get_type(),
    }
}

impl X11Screen {
    pub fn get_current_desktop(&self) -> u32 {
        unsafe { gdk_x11_sys::gdk_x11_screen_get_current_desktop(self.to_glib_none().0) }
    }

    pub fn get_monitor_output(&self, monitor_num: i32) -> xlib::XID {
        unsafe {
            gdk_x11_sys::gdk_x11_screen_get_monitor_output(self.to_glib_none().0, monitor_num)
        }
    }

    pub fn get_number_of_desktops(&self) -> u32 {
        unsafe { gdk_x11_sys::gdk_x11_screen_get_number_of_desktops(self.to_glib_none().0) }
    }

    pub fn get_screen_number(&self) -> i32 {
        unsafe { gdk_x11_sys::gdk_x11_screen_get_screen_number(self.to_glib_none().0) }
    }

    pub fn get_window_manager_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gdk_x11_sys::gdk_x11_screen_get_window_manager_name(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn lookup_visual(&self, xvisualid: xlib::VisualID) -> Option<X11Visual> {
        unsafe {
            from_glib_none(gdk_x11_sys::gdk_x11_screen_lookup_visual(
                self.to_glib_none().0,
                xvisualid,
            ))
        }
    }

    pub fn supports_net_wm_hint(&self, property: &gdk::Atom) -> bool {
        unsafe {
            from_glib(gdk_x11_sys::gdk_x11_screen_supports_net_wm_hint(
                self.to_glib_none().0,
                property.to_glib_none().0,
            ))
        }
    }

    pub fn connect_window_manager_changed<F: Fn(&X11Screen) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn window_manager_changed_trampoline<F: Fn(&X11Screen) + 'static>(
            this: *mut gdk_x11_sys::GdkX11Screen,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"window-manager-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    window_manager_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for X11Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11Screen")
    }
}
