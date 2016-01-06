// This file was generated by gir (d4a8bb5) from gir-files (11e0e6d)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct TreePath(Boxed<ffi::GtkTreePath>);

    match fn {
        copy => |ptr| ffi::gtk_tree_path_copy(mut_override(ptr)),
        free => |ptr| ffi::gtk_tree_path_free(ptr),
    }
}

impl TreePath {
    pub fn new() -> TreePath {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_tree_path_new())
        }
    }

    pub fn new_first() -> TreePath {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_tree_path_new_first())
        }
    }

    //pub fn new_from_indices(first_index: i32, : /*Unknown conversion*/Fundamental: VarArgs) -> TreePath {
    //    unsafe { TODO: call ffi::gtk_tree_path_new_from_indices() }
    //}

    //#[cfg(gtk_3_12)]
    //pub fn new_from_indicesv(indices: /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 0, id: 14 }", length: Fundamental: Size) -> TreePath {
    //    unsafe { TODO: call ffi::gtk_tree_path_new_from_indicesv() }
    //}

    pub fn new_from_string(path: &str) -> TreePath {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_tree_path_new_from_string(path.to_glib_none().0))
        }
    }

    pub fn append_index(&mut self, index_: i32) {
        unsafe {
            ffi::gtk_tree_path_append_index(self.to_glib_none_mut().0, index_);
        }
    }

    pub fn compare(&self, b: &TreePath) -> i32 {
        unsafe {
            ffi::gtk_tree_path_compare(self.to_glib_none().0, b.to_glib_none().0)
        }
    }

    pub fn down(&mut self) {
        unsafe {
            ffi::gtk_tree_path_down(self.to_glib_none_mut().0);
        }
    }

    pub fn get_depth(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_path_get_depth(mut_override(self.to_glib_none().0))
        }
    }

    //pub fn get_indices_with_depth(&mut self, depth: &mut i32) -> /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 0, id: 14 }" {
    //    unsafe { TODO: call ffi::gtk_tree_path_get_indices_with_depth() }
    //}

    pub fn is_ancestor(&self, descendant: &TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_path_is_ancestor(mut_override(self.to_glib_none().0), mut_override(descendant.to_glib_none().0)))
        }
    }

    pub fn is_descendant(&self, ancestor: &TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_path_is_descendant(mut_override(self.to_glib_none().0), mut_override(ancestor.to_glib_none().0)))
        }
    }

    pub fn next(&mut self) {
        unsafe {
            ffi::gtk_tree_path_next(self.to_glib_none_mut().0);
        }
    }

    pub fn prepend_index(&mut self, index_: i32) {
        unsafe {
            ffi::gtk_tree_path_prepend_index(self.to_glib_none_mut().0, index_);
        }
    }

    pub fn prev(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_path_prev(self.to_glib_none_mut().0))
        }
    }

    pub fn to_string(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_tree_path_to_string(mut_override(self.to_glib_none().0)))
        }
    }

    pub fn up(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_path_up(self.to_glib_none_mut().0))
        }
    }

}
