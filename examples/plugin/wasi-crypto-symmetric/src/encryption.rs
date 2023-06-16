use std::ptr::null;

mod ffi {

    #[link(wasm_import_module = "wasi_ephemeral_crypto_symmetric")]
    extern "C" {
        pub fn symmetric_key_generate(
            AlgPtr: *const u8,
            AlgLen: u32,
            OptOptionsPtr: *const u8,
            KeyHandlePtr: *mut u32,
        ) -> i32;
        pub fn symmetric_key_import(
            AlgPtr: *const u8,
            AlgLen: u32,
            RawPtr: *const u8,
            RawLen: u32,
            KeyHandlePtr: *mut u32,
        ) -> i32;
        pub fn symmetric_key_export(KeyHandle: u32, ArrayOutputHandlePtr: *mut u32) -> i32;
    }

    #[link(wasm_import_module = "wasi_ephemeral_crypto_common")]
    extern "C" {
        pub fn array_output_len(ArrayOutputHandle: u32, SizePtr: *mut u32) -> i32;
        pub fn array_output_pull(
            ArrayOutputHandle: u32,
            BufPtr: *mut u8,
            BufLen: u32,
            SizePtr: *mut u32,
        ) -> i32;
    }
}

pub fn symmetric_key_generate<S: AsRef<[u8]>>(alg: &str) -> Result<u32, i32> {
    let alg_size = alg.len() as u32;
    let mut key_handle: u32 = 0;
    unsafe {
        let res =
            ffi::symmetric_key_generate(alg.as_ptr(), alg_size, null(), &mut key_handle);
        println!("{:?}", key_handle);
        if res == 0 {
            Ok(key_handle)
        } else {
            Err(res)
        }
    }
}

pub fn symmetric_key_import<S: AsRef<[u8]>>(
    alg: String,
    raw: S,
) -> Result<u32, i32> {
    let alg_size = alg.len() as u32;
    let raw_size = raw.as_ref().len() as u32;
    let mut key_handle: u32 = 0;
    unsafe {
        let res = ffi::symmetric_key_import(
            alg.as_ptr(),
            alg_size,
            raw.as_ref().as_ptr(),
            raw_size,
            &mut key_handle,
        );
        if res == 0 {
            Ok(key_handle)
        } else {
            Err(res)
        }
    }
}


pub fn symmetric_key_export(key_handle: u32, encoding: u16) -> Result<u32, i32> {
    let mut output_handle: u32 = 0;
    unsafe {
        let res = ffi::symmetric_key_export(key_handle, &mut output_handle);
        if res == 0 {
            Ok(output_handle)
        } else {
            Err(res)
        }
    }
}

pub fn array_output_len(output_handle: u32) -> Result<u32, i32> {
    let mut length: u32 = 0;
    unsafe {
        let res = ffi::array_output_len(output_handle, &mut length);
        if res == 0 {
            Ok(length)
        } else {
            Err(res)
        }
    }
}

pub fn array_output_pull(output_handle: u32, buf: &mut Vec<u8>) -> Result<u32, i32> {
    let mut length: u32 = 0;
    unsafe {
        let res = ffi::array_output_pull(output_handle, buf.as_mut_ptr(), 64, &mut length);
        if res == 0 {
            Ok(length)
        } else {
            Err(res)
        }
    }
}
