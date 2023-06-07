// Generated by `wit-bindgen` 0.7.0. DO NOT EDIT!

/// Declares the export of the component's world for the
/// given type.
#[macro_export]
macro_rules! export_invoker_grpc(($t:ident) => {
  const _: () = {

    #[doc(hidden)]
    #[export_name = "gprc#call"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __export_call(arg0: i32,arg1: i32,) -> i32 {
      exports::gprc::call_call::<$t>(arg0,arg1,)
    }

    #[doc(hidden)]
    #[export_name = "cabi_post_gprc#call"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __post_return_call(arg0: i32,) {
      exports::gprc::post_return_call::<$t>(arg0,)
    }

  };

  #[used]
  #[doc(hidden)]
  #[cfg(target_arch = "wasm32")]
  static __FORCE_SECTION_REF: fn() = __link_section;
});
pub mod exports {

    #[allow(clippy::all)]
    pub mod gprc {
        #[used]
        #[doc(hidden)]
        #[cfg(target_arch = "wasm32")]
        static __FORCE_SECTION_REF: fn() = super::super::__link_section;

        pub type Error = u32;
        pub trait Gprc {
            fn call(
                payload: wit_bindgen::rt::string::String,
            ) -> Result<wit_bindgen::rt::string::String, Error>;
        }

        #[doc(hidden)]
        pub unsafe fn call_call<T: Gprc>(arg0: i32, arg1: i32) -> i32 {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, string::String, vec::Vec};

            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch = "wasm32")]
            wit_bindgen::rt::run_ctors_once();

            let len0 = arg1 as usize;
            let result1 = T::call({
                #[cfg(not(debug_assertions))]
                {
                    String::from_utf8_unchecked(Vec::from_raw_parts(arg0 as *mut _, len0, len0))
                }
                #[cfg(debug_assertions)]
                {
                    String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap()
                }
            });
            let ptr2 = _RET_AREA.0.as_mut_ptr() as i32;
            match result1 {
                Ok(e) => {
                    *((ptr2 + 0) as *mut u8) = (0i32) as u8;
                    let vec3 = (e.into_bytes()).into_boxed_slice();
                    let ptr3 = vec3.as_ptr() as i32;
                    let len3 = vec3.len() as i32;
                    ::core::mem::forget(vec3);
                    *((ptr2 + 8) as *mut i32) = len3;
                    *((ptr2 + 4) as *mut i32) = ptr3;
                }
                Err(e) => {
                    *((ptr2 + 0) as *mut u8) = (1i32) as u8;
                    *((ptr2 + 4) as *mut i32) = wit_bindgen::rt::as_i32(e);
                }
            };
            ptr2
        }

        #[doc(hidden)]
        pub unsafe fn post_return_call<T: Gprc>(arg0: i32) {
            match i32::from(*((arg0 + 0) as *const u8)) {
                0 => {
                    wit_bindgen::rt::dealloc(
                        *((arg0 + 4) as *const i32),
                        (*((arg0 + 8) as *const i32)) as usize,
                        1,
                    );
                }
                _ => (),
            }
        }

        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, string::String, vec::Vec};

        #[repr(align(4))]
        struct _RetArea([u8; 12]);
        static mut _RET_AREA: _RetArea = _RetArea([0; 12]);
    }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:invoker-grpc"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 214] = [
    3, 0, 12, 105, 110, 118, 111, 107, 101, 114, 45, 103, 114, 112, 99, 0, 97, 115, 109, 13, 0, 1,
    0, 7, 92, 1, 65, 2, 1, 65, 2, 1, 66, 5, 1, 121, 4, 0, 5, 101, 114, 114, 111, 114, 3, 0, 0, 1,
    106, 1, 115, 1, 1, 1, 64, 1, 7, 112, 97, 121, 108, 111, 97, 100, 115, 0, 2, 4, 0, 4, 99, 97,
    108, 108, 1, 3, 4, 0, 4, 103, 112, 114, 99, 5, 0, 4, 1, 27, 101, 103, 99, 99, 114, 105, 58,
    105, 110, 118, 111, 107, 101, 114, 47, 105, 110, 118, 111, 107, 101, 114, 45, 103, 114, 112,
    99, 4, 0, 0, 69, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101,
    115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101,
    110, 116, 6, 48, 46, 49, 49, 46, 48, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110,
    45, 114, 117, 115, 116, 5, 48, 46, 55, 46, 48, 11, 24, 1, 1, 18, 101, 103, 99, 99, 114, 105,
    58, 105, 110, 118, 111, 107, 101, 114, 47, 119, 105, 116, 3, 0, 0,
];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}

pub use export_invoker_grpc;
