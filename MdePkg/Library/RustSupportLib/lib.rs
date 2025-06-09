#![no_std]
#![feature(c_variadic)]
#![feature(extended_varargs_abi_support)]

#[allow(dead_code)]
mod efi_bindings {
    type EFI_STATUS = ::core::ffi::c_ulonglong;
    type EFI_HANDLE = *mut ::core::ffi::c_void;
    type EFI_INSTALL_MULTIPLE_PROTOCOL_INTERFACES = ::core::option::Option<
        unsafe extern "win64" fn(
            Handle: *mut EFI_HANDLE,
            ...
        ) -> EFI_STATUS,
    >; 
    type EFI_UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES = ::core::option::Option<
        unsafe extern "win64" fn(
            Handle: EFI_HANDLE,
            ...
        ) -> EFI_STATUS,
    >; 

    #[repr(C)]
    #[derive(Copy, Clone)]
    union EFI_BOOT_KEY_DATA {
        pub PackedValue: ::core::ffi::c_uint,
    }

    include!("bindings.rs");
}

pub use efi_bindings::*;