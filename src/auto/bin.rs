// This file was generated by gir (d4a8bb5) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use Container;
use Widget;
use ffi;
use glib::object::Upcast;
use glib::translate::*;

glib_wrapper! {
    pub struct Bin(Object<ffi::GtkBin>): Widget, Container, Buildable;

    match fn {
        get_type => || ffi::gtk_bin_get_type(),
    }
}

pub trait BinExt {
    fn get_child(&self) -> Option<Widget>;
}

impl<O: Upcast<Bin>> BinExt for O {
    fn get_child(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_bin_get_child(self.to_glib_none().0))
        }
    }

}
