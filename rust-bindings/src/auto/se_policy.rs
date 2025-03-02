// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Repo;
use crate::SePolicyRestoreconFlags;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::StaticType;
use std::fmt;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "OstreeSePolicy")]
    pub struct SePolicy(Object<ffi::OstreeSePolicy>);

    match fn {
        type_ => || ffi::ostree_sepolicy_get_type(),
    }
}

impl SePolicy {
    #[doc(alias = "ostree_sepolicy_new")]
    pub fn new(path: &impl IsA<gio::File>, cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<SePolicy, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ostree_sepolicy_new(path.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2017_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2017_4")))]
    #[doc(alias = "ostree_sepolicy_new_at")]
    pub fn new_at(rootfs_dfd: i32, cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<SePolicy, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ostree_sepolicy_new_at(rootfs_dfd, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_sepolicy_new_from_commit")]
    #[doc(alias = "new_from_commit")]
    pub fn from_commit(repo: &Repo, rev: &str, cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<SePolicy, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ostree_sepolicy_new_from_commit(repo.to_glib_none().0, rev.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2016_5", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2016_5")))]
    #[doc(alias = "ostree_sepolicy_get_csum")]
    #[doc(alias = "get_csum")]
    pub fn csum(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ostree_sepolicy_get_csum(self.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_sepolicy_get_label")]
    #[doc(alias = "get_label")]
    pub fn label(&self, relpath: &str, unix_mode: u32, cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut out_label = ptr::null_mut();
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_sepolicy_get_label(self.to_glib_none().0, relpath.to_glib_none().0, unix_mode, &mut out_label, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(from_glib_full(out_label)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_sepolicy_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ostree_sepolicy_get_name(self.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_sepolicy_get_path")]
    #[doc(alias = "get_path")]
    pub fn path(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(ffi::ostree_sepolicy_get_path(self.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_sepolicy_restorecon")]
    pub fn restorecon(&self, path: &str, info: Option<&gio::FileInfo>, target: &impl IsA<gio::File>, flags: SePolicyRestoreconFlags, cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut out_new_label = ptr::null_mut();
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_sepolicy_restorecon(self.to_glib_none().0, path.to_glib_none().0, info.to_glib_none().0, target.as_ref().to_glib_none().0, flags.into_glib(), &mut out_new_label, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(from_glib_full(out_new_label)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_sepolicy_setfscreatecon")]
    pub fn setfscreatecon(&self, path: &str, mode: u32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_sepolicy_setfscreatecon(self.to_glib_none().0, path.to_glib_none().0, mode, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "rootfs-dfd")]
    pub fn rootfs_dfd(&self) -> i32 {
        glib::ObjectExt::property(self, "rootfs-dfd")
    }
}

unsafe impl Send for SePolicy {}

impl fmt::Display for SePolicy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SePolicy")
    }
}
