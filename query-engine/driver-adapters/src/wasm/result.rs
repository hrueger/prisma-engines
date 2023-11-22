use js_sys::Boolean as JsBoolean;
use quaint::error::{Error as QuaintError, ErrorKind};
use serde::de::DeserializeOwned;
use wasm_bindgen::{JsCast, JsValue};

use crate::{error::DriverAdapterError, JsObjectExtern};

impl From<DriverAdapterError> for QuaintError {
    fn from(value: DriverAdapterError) -> Self {
        match value {
            DriverAdapterError::UnsupportedNativeDataType { native_type } => {
                QuaintError::builder(ErrorKind::UnsupportedColumnType {
                    column_type: native_type,
                })
                .build()
            }
            DriverAdapterError::GenericJs { id } => QuaintError::external_error(id),
            DriverAdapterError::Postgres(e) => e.into(),
            DriverAdapterError::Mysql(e) => e.into(),
            DriverAdapterError::Sqlite(e) => e.into(),
            // in future, more error types would be added and we'll need to convert them to proper QuaintErrors here
        }
    }
}

/// Wrapper for JS-side result type
pub(crate) enum JsResult<T>
where
    T: DeserializeOwned,
{
    Ok(T),
    Err(DriverAdapterError),
}

impl<T> TryFrom<JsValue> for JsResult<T>
where
    T: DeserializeOwned,
{
    type Error = JsValue;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        Self::from_js_unknown(value)
    }
}

impl<T> JsResult<T>
where
    T: DeserializeOwned,
{
    fn from_js_unknown(unknown: JsValue) -> Result<Self, JsValue> {
        let object = unknown.unchecked_into::<JsObjectExtern>();

        let ok: JsBoolean = object.get("ok".into())?.unchecked_into();
        let ok = ok.value_of();

        if ok {
            let js_value: JsValue = object.get("value".into())?;
            let deserialized = serde_wasm_bindgen::from_value::<T>(js_value)?;
            return Ok(Self::Ok(deserialized));
        }

        let error = object.get("error".into())?;
        let error: DriverAdapterError = serde_wasm_bindgen::from_value(error)?;
        Ok(Self::Err(error))
    }
}

impl<T> From<JsResult<T>> for quaint::Result<T>
where
    T: DeserializeOwned,
{
    fn from(value: JsResult<T>) -> Self {
        match value {
            JsResult::Ok(result) => Ok(result),
            JsResult::Err(error) => Err(error.into()),
        }
    }
}
