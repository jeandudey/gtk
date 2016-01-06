// This file was generated by gir (d4a8bb5) from gir-files (11e0e6d)
// DO NOT EDIT

use RecentFilter;
use RecentInfo;
use RecentSortType;
use ffi;
use glib::object::Upcast;
use glib::translate::*;

glib_wrapper! {
    pub struct RecentChooser(Object<ffi::GtkRecentChooser>);

    match fn {
        get_type => || ffi::gtk_recent_chooser_get_type(),
    }
}

pub trait RecentChooserExt {
    fn add_filter(&self, filter: &RecentFilter);
    fn get_current_item(&self) -> RecentInfo;
    fn get_current_uri(&self) -> Option<String>;
    fn get_filter(&self) -> Option<RecentFilter>;
    fn get_items(&self) -> Vec<RecentInfo>;
    fn get_limit(&self) -> i32;
    fn get_local_only(&self) -> bool;
    fn get_select_multiple(&self) -> bool;
    fn get_show_icons(&self) -> bool;
    fn get_show_not_found(&self) -> bool;
    fn get_show_private(&self) -> bool;
    fn get_show_tips(&self) -> bool;
    fn get_sort_type(&self) -> RecentSortType;
    //fn get_uris(&self, length: Fundamental: Size) -> /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 0, id: 28 }";
    //fn list_filters(&self) -> /*Unknown conversion*/Unknown rust type: "SList TypeId { ns_id: 1, id: 699 }";
    fn remove_filter(&self, filter: &RecentFilter);
    fn select_all(&self);
    //fn select_uri(&self, uri: &str, error: /*Ignored*/Option<glib::Error>) -> bool;
    //fn set_current_uri(&self, uri: &str, error: /*Ignored*/Option<glib::Error>) -> bool;
    fn set_filter(&self, filter: Option<&RecentFilter>);
    fn set_limit(&self, limit: i32);
    fn set_local_only(&self, local_only: bool);
    fn set_select_multiple(&self, select_multiple: bool);
    fn set_show_icons(&self, show_icons: bool);
    fn set_show_not_found(&self, show_not_found: bool);
    fn set_show_private(&self, show_private: bool);
    fn set_show_tips(&self, show_tips: bool);
    //fn set_sort_func(&self, sort_func: /*Unknown conversion*/Unknown rust type: "RecentSortFunc", sort_data: Option<Fundamental: Pointer>, data_destroy: /*Unknown conversion*/Unknown rust type: "DestroyNotify");
    fn set_sort_type(&self, sort_type: RecentSortType);
    fn unselect_all(&self);
    fn unselect_uri(&self, uri: &str);
}

impl<O: Upcast<RecentChooser>> RecentChooserExt for O {
    fn add_filter(&self, filter: &RecentFilter) {
        unsafe {
            ffi::gtk_recent_chooser_add_filter(self.to_glib_none().0, filter.to_glib_none().0);
        }
    }

    fn get_current_item(&self) -> RecentInfo {
        unsafe {
            from_glib_full(ffi::gtk_recent_chooser_get_current_item(self.to_glib_none().0))
        }
    }

    fn get_current_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_recent_chooser_get_current_uri(self.to_glib_none().0))
        }
    }

    fn get_filter(&self) -> Option<RecentFilter> {
        unsafe {
            from_glib_none(ffi::gtk_recent_chooser_get_filter(self.to_glib_none().0))
        }
    }

    fn get_items(&self) -> Vec<RecentInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_recent_chooser_get_items(self.to_glib_none().0))
        }
    }

    fn get_limit(&self) -> i32 {
        unsafe {
            ffi::gtk_recent_chooser_get_limit(self.to_glib_none().0)
        }
    }

    fn get_local_only(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_local_only(self.to_glib_none().0))
        }
    }

    fn get_select_multiple(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_select_multiple(self.to_glib_none().0))
        }
    }

    fn get_show_icons(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_show_icons(self.to_glib_none().0))
        }
    }

    fn get_show_not_found(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_show_not_found(self.to_glib_none().0))
        }
    }

    fn get_show_private(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_show_private(self.to_glib_none().0))
        }
    }

    fn get_show_tips(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_show_tips(self.to_glib_none().0))
        }
    }

    fn get_sort_type(&self) -> RecentSortType {
        unsafe {
            ffi::gtk_recent_chooser_get_sort_type(self.to_glib_none().0)
        }
    }

    //fn get_uris(&self, length: Fundamental: Size) -> /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 0, id: 28 }" {
    //    unsafe { TODO: call ffi::gtk_recent_chooser_get_uris() }
    //}

    //fn list_filters(&self) -> /*Unknown conversion*/Unknown rust type: "SList TypeId { ns_id: 1, id: 699 }" {
    //    unsafe { TODO: call ffi::gtk_recent_chooser_list_filters() }
    //}

    fn remove_filter(&self, filter: &RecentFilter) {
        unsafe {
            ffi::gtk_recent_chooser_remove_filter(self.to_glib_none().0, filter.to_glib_none().0);
        }
    }

    fn select_all(&self) {
        unsafe {
            ffi::gtk_recent_chooser_select_all(self.to_glib_none().0);
        }
    }

    //fn select_uri(&self, uri: &str, error: /*Ignored*/Option<glib::Error>) -> bool {
    //    unsafe { TODO: call ffi::gtk_recent_chooser_select_uri() }
    //}

    //fn set_current_uri(&self, uri: &str, error: /*Ignored*/Option<glib::Error>) -> bool {
    //    unsafe { TODO: call ffi::gtk_recent_chooser_set_current_uri() }
    //}

    fn set_filter(&self, filter: Option<&RecentFilter>) {
        unsafe {
            ffi::gtk_recent_chooser_set_filter(self.to_glib_none().0, filter.to_glib_none().0);
        }
    }

    fn set_limit(&self, limit: i32) {
        unsafe {
            ffi::gtk_recent_chooser_set_limit(self.to_glib_none().0, limit);
        }
    }

    fn set_local_only(&self, local_only: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_local_only(self.to_glib_none().0, local_only.to_glib());
        }
    }

    fn set_select_multiple(&self, select_multiple: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_select_multiple(self.to_glib_none().0, select_multiple.to_glib());
        }
    }

    fn set_show_icons(&self, show_icons: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_icons(self.to_glib_none().0, show_icons.to_glib());
        }
    }

    fn set_show_not_found(&self, show_not_found: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_not_found(self.to_glib_none().0, show_not_found.to_glib());
        }
    }

    fn set_show_private(&self, show_private: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_private(self.to_glib_none().0, show_private.to_glib());
        }
    }

    fn set_show_tips(&self, show_tips: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_tips(self.to_glib_none().0, show_tips.to_glib());
        }
    }

    //fn set_sort_func(&self, sort_func: /*Unknown conversion*/Unknown rust type: "RecentSortFunc", sort_data: Option<Fundamental: Pointer>, data_destroy: /*Unknown conversion*/Unknown rust type: "DestroyNotify") {
    //    unsafe { TODO: call ffi::gtk_recent_chooser_set_sort_func() }
    //}

    fn set_sort_type(&self, sort_type: RecentSortType) {
        unsafe {
            ffi::gtk_recent_chooser_set_sort_type(self.to_glib_none().0, sort_type);
        }
    }

    fn unselect_all(&self) {
        unsafe {
            ffi::gtk_recent_chooser_unselect_all(self.to_glib_none().0);
        }
    }

    fn unselect_uri(&self, uri: &str) {
        unsafe {
            ffi::gtk_recent_chooser_unselect_uri(self.to_glib_none().0, uri.to_glib_none().0);
        }
    }

}
