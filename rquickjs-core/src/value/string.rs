use crate::{handle_exception, qjs, Ctx, Error, Result, StdString, Value};
use std::{
    ffi::{CStr, CString},
    mem, slice, str,
};

/// Rust representation of a javascript string.
#[derive(Debug, Clone, PartialEq)]
#[repr(transparent)]
pub struct String<'js>(pub(crate) Value<'js>);

impl<'js> String<'js> {
    pub fn to_cstring(&self) -> Result<CString> {
        let ptr = unsafe { qjs::JS_ToCString(self.0.ctx.ctx, self.0.as_js_value()) };
        if ptr.is_null() {
            // Might not ever happen but I am not 100% sure
            // so just incase check it.
            return Err(Error::Unknown);
        }
        let result = unsafe { CString::from_raw(ptr as *mut i8) };
        Ok(result)
    }

    /// Convert the javascript string to a rust string.
    pub fn to_string(&self) -> Result<StdString> {
        let mut len = mem::MaybeUninit::uninit();
        let ptr =
            unsafe { qjs::JS_ToCStringLen(self.0.ctx.ctx, len.as_mut_ptr(), self.0.as_js_value()) };
        if ptr.is_null() {
            // Might not ever happen but I am not 100% sure
            // so just incase check it.
            return Err(Error::Unknown);
        }
        let len = unsafe { len.assume_init() };
        let bytes: &[u8] = unsafe { slice::from_raw_parts(ptr as _, len as _) };
        let result = str::from_utf8(bytes).map(|s| s.into());
        unsafe { qjs::JS_FreeCString(self.0.ctx.ctx, ptr) };
        Ok(result?)
    }

    /// Create a new js string from an rust string.
    pub fn from_str(ctx: Ctx<'js>, s: &str) -> Result<Self> {
        let len = s.as_bytes().len();
        let ptr = s.as_ptr();
        Ok(unsafe {
            let js_val = qjs::JS_NewStringLen(ctx.ctx, ptr as _, len as _);
            let js_val = handle_exception(ctx, js_val)?;
            String::from_js_value(ctx, js_val)
        })
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn from_javascript() {
        test_with(|ctx| {
            let s: String = ctx.eval(" 'foo bar baz' ").unwrap();
            assert_eq!(s.to_string().unwrap(), "foo bar baz");
        });
    }

    #[test]
    fn to_javascript() {
        test_with(|ctx| {
            let string = String::from_str(ctx, "foo").unwrap();
            let func: Function = ctx.eval("x =>  x + 'bar'").unwrap();
            let text: StdString = (string,).apply(&func).unwrap();
            assert_eq!(text, "foobar".to_string());
        });
    }
}
