use crate::names::Names;

use proc_macro2::TokenStream;
use quote::quote;
use witx::Layout;

pub(super) fn define_handle(
    names: &Names,
    name: &witx::Id,
    h: &witx::HandleDatatype,
) -> TokenStream {
    let ident = names.type_(name);
    let size = h.mem_size_align().size as u32;
    let align = h.mem_size_align().align as usize;
    quote! {
        #[derive(Debug, PartialEq)]
        pub struct #ident(u32);

        impl #ident {
            pub unsafe fn from_raw(raw: u32) -> Self {
                Self(raw)
            }
            pub fn as_raw(&self) -> u32 {
                self.0
            }
        }

        impl Clone for #ident {
            fn clone(&self) -> Self {
                Self(self.0)
            }
        }

        impl From<#ident> for i32 {
            fn from(e: #ident) -> i32 {
                e.0 as i32
            }
        }

        impl From<i32> for #ident {
            fn from(e: i32) -> #ident {
                #ident(e as u32)
            }
        }

        impl ::std::fmt::Display for #ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}({})", stringify!(#ident), self.0)
            }
        }

        impl<'a> wiggle_runtime::GuestType<'a> for #ident {
            fn guest_size() -> u32 {
                #size
            }

            fn guest_align() -> usize {
                #align
            }

            fn read(location: &wiggle_runtime::GuestPtr<'a, #ident>) -> Result<#ident, wiggle_runtime::GuestError> {
                Ok(#ident(u32::read(&location.cast())?))
            }

            fn write(location: &wiggle_runtime::GuestPtr<'_, Self>, val: Self) -> Result<(), wiggle_runtime::GuestError> {
                u32::write(&location.cast(), val.0)
            }
        }

        unsafe impl<'a> wiggle_runtime::GuestTypeTransparent<'a> for #ident {
            #[inline]
            fn validate(_location: *mut #ident) -> Result<(), wiggle_runtime::GuestError> {
                // All bit patterns accepted
                Ok(())
            }
        }


    }
}
