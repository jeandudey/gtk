// This file was generated by gir (d4a8bb5) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Button;
use CheckButton;
use Container;
use ToggleButton;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct RadioButton(Object<ffi::GtkRadioButton>): Widget, Container, Bin, Button, ToggleButton, CheckButton, Actionable, Buildable;

    match fn {
        get_type => || ffi::gtk_radio_button_get_type(),
    }
}

impl RadioButton {
    //pub fn new(group: /*Unknown conversion*/Unknown rust type: "SList TypeId { ns_id: 1, id: 665 }") -> RadioButton {
    //    unsafe { TODO: call ffi::gtk_radio_button_new() }
    //}

    pub fn new_from_widget(radio_group_member: Option<&RadioButton>) -> RadioButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_from_widget(radio_group_member.to_glib_none().0)).downcast_unchecked()
        }
    }

    //pub fn new_with_label(group: /*Unknown conversion*/Unknown rust type: "SList TypeId { ns_id: 1, id: 665 }", label: &str) -> RadioButton {
    //    unsafe { TODO: call ffi::gtk_radio_button_new_with_label() }
    //}

    pub fn new_with_label_from_widget(radio_group_member: Option<&RadioButton>, label: &str) -> RadioButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_with_label_from_widget(radio_group_member.to_glib_none().0, label.to_glib_none().0)).downcast_unchecked()
        }
    }

    //pub fn new_with_mnemonic(group: /*Unknown conversion*/Unknown rust type: "SList TypeId { ns_id: 1, id: 665 }", label: &str) -> RadioButton {
    //    unsafe { TODO: call ffi::gtk_radio_button_new_with_mnemonic() }
    //}

    pub fn new_with_mnemonic_from_widget(radio_group_member: Option<&RadioButton>, label: &str) -> RadioButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_with_mnemonic_from_widget(radio_group_member.to_glib_none().0, label.to_glib_none().0)).downcast_unchecked()
        }
    }

    //pub fn get_group(&self) -> /*Unknown conversion*/Unknown rust type: "SList TypeId { ns_id: 1, id: 665 }" {
    //    unsafe { TODO: call ffi::gtk_radio_button_get_group() }
    //}

    pub fn join_group(&self, group_source: Option<&RadioButton>) {
        unsafe {
            ffi::gtk_radio_button_join_group(self.to_glib_none().0, group_source.to_glib_none().0);
        }
    }

    //pub fn set_group(&self, group: /*Unknown conversion*/Unknown rust type: "SList TypeId { ns_id: 1, id: 665 }") {
    //    unsafe { TODO: call ffi::gtk_radio_button_set_group() }
    //}

}
