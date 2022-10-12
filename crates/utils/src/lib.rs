use neon::prelude::*;

fn get_object_keys<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> JsResult<'a, JsArray> {
    obj.get_own_property_names(cx)
}

fn js_value_to_object<'a>(
    cx: &mut impl Context<'a>,
    value: Handle<'a, JsValue>,
) -> JsResult<'a, JsObject> {
    value.downcast(cx).or_throw(cx)
}

fn js_value_to_date<'a>(
    cx: &mut impl Context<'a>,
    value: Handle<'a, JsValue>,
) -> JsResult<'a, neon::types::JsDate> {
    value.downcast(cx).or_throw(cx)
}

fn js_value_to_string<'a>(
    cx: &mut impl Context<'a>,
    value: Handle<'a, JsValue>,
) -> JsResult<'a, JsString> {
    value.downcast(cx).or_throw(cx)
}

fn is_object_empty<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> bool {
    let keys: Handle<JsArray> = get_object_keys(cx, obj).unwrap();
    return keys.is_empty(cx);
}

fn is_object_and_empty<'a>(
    cx: &mut impl Context<'a>,
    val: Handle<'a, JsValue>,
) -> bool {
    if is_object(cx, val) == false {
        return false;
    }

    let obj = js_value_to_object(cx, val).unwrap();

    let is_empty = is_object_empty(cx, obj);

    return is_empty;
}

fn is_undefined<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsValue>,
) -> bool {
    return obj.is_a::<JsUndefined, _>(cx);
}

fn is_object<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsValue>,
) -> bool {
    return obj.is_a::<JsObject, _>(cx);
}

fn is_date<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsValue>,
) -> bool {
    return obj.is_a::<neon::types::JsDate, _>(cx);
}

fn added_diff<'a>(
    cx: &mut impl Context<'a>,
    lhs: Handle<'a, JsValue>,
    rhs: Handle<'a, JsValue>,
) -> JsResult<'a, JsValue> {

    let acc = cx.empty_object();

    if rhs.strict_equals(cx, lhs) == false
        && is_object(cx, rhs) == true
        && is_object(cx, lhs) == true {

        let r: Handle<JsObject> = js_value_to_object(cx, rhs)?;
        let l: Handle<JsObject> = js_value_to_object(cx, lhs)?;

        let r_keys: Handle<JsArray> = get_object_keys(cx, r)?;
        let r_keys_vec: Vec<Handle<JsValue>> = r_keys.to_vec(cx)?;

        for r_key in r_keys_vec.iter() {

            let key_as_string = js_value_to_string(cx, *r_key)?;

            let r_val: Handle<JsValue> = r.get_value(cx, key_as_string)?;
            let l_val: Handle<JsValue> = l.get_value(cx, key_as_string)?;

            if is_undefined(cx, l_val) == false {

                let diff: Handle<JsValue> = added_diff(cx, l_val, r_val)?;

                if is_object_and_empty(cx, diff) == false {
                    acc.set(cx, key_as_string, diff)?;
                }

            } else {
                acc.set(cx, key_as_string, r_val)?;
            }
        }

    }

    Ok(acc.upcast())
}

fn deleted_diff<'a>(
    cx: &mut impl Context<'a>,
    lhs: Handle<'a, JsValue>,
    rhs: Handle<'a, JsValue>,
) -> JsResult<'a, JsObject> {
    if rhs.strict_equals(cx, lhs) {
        return Ok(cx.empty_object());
    }

    if is_object(cx, rhs) == false {
        return Ok(cx.empty_object());
    }

    if is_object(cx, lhs) == false {
        return Ok(cx.empty_object());
    }

    let r: Handle<JsObject> = js_value_to_object(cx, rhs)?;
    let l: Handle<JsObject> = js_value_to_object(cx, lhs)?;

    let r_keys: Handle<JsArray> = get_object_keys(cx, r)?;
    let l_keys: Handle<JsArray> = get_object_keys(cx, l)?;

    let r_keys_vec: Vec<Handle<JsValue>> = r_keys.to_vec(cx)?;
    let l_keys_vec: Vec<Handle<JsValue>> = l_keys.to_vec(cx)?;

    let acc = cx.empty_object();

    for (_i, l_key) in l_keys_vec.iter().enumerate() {
        let mut r_has_l_key: bool = false;
        let key_as_string = js_value_to_string(cx, *l_key)?;

        for (_i, r_key) in r_keys_vec.iter().enumerate() {
            let r_key_as_string = js_value_to_string(cx, *r_key)?;
            if r_key_as_string.strict_equals(cx, key_as_string) {
                r_has_l_key = true;
            }
        }

        let l_val: Handle<JsValue> = l.get(cx, key_as_string)?;
        if r_has_l_key {
            let r_val: Handle<JsValue> = r.get(cx, key_as_string)?;
            let diff: Handle<JsObject> = deleted_diff(cx, l_val, r_val)?;

            let is_diff_empty = is_object_empty(cx, diff);
            if is_diff_empty == false {
                acc.set(cx, key_as_string, diff)?;
            }
        } else {
            let undefined_value = cx.undefined();
            acc.set(cx, key_as_string, undefined_value)?;
        }
    }

    Ok(acc)
}

fn updated_diff<'a>(
    cx: &mut impl Context<'a>,
    lhs: Handle<'a, JsValue>,
    rhs: Handle<'a, JsValue>,
) -> JsResult<'a, JsValue> {
    if rhs.strict_equals(cx, lhs) {
        return Ok(cx.empty_object().upcast());
    }

    if is_date(cx, lhs) && is_date(cx, rhs) {
        let lhs_as_date = js_value_to_date(cx, lhs)?;
        let rhs_as_date = js_value_to_date(cx, rhs)?;
        if lhs_as_date.value(cx) == rhs_as_date.value(cx) {
            return Ok(cx.empty_object().upcast());
        }
    }

    if is_date(cx, lhs) || is_date(cx, rhs) {
        return Ok(rhs);
    }

    if is_object(cx, lhs) == false || is_object(cx, rhs) == false {
        return Ok(rhs);
    }

    let l: Handle<JsObject> = js_value_to_object(cx, lhs)?;
    let r: Handle<JsObject> = js_value_to_object(cx, rhs)?;

    let l_keys: Handle<JsArray> = get_object_keys(cx, l)?;
    let r_keys: Handle<JsArray> = get_object_keys(cx, r)?;

    let l_keys_vec: Vec<Handle<JsValue>> = l_keys.to_vec(cx)?;
    let r_keys_vec: Vec<Handle<JsValue>> = r_keys.to_vec(cx)?;

    let acc = cx.empty_object();

    for (_i, r_key) in r_keys_vec.iter().enumerate() {
        let mut has_key: bool = false;
        let key_as_string = js_value_to_string(cx, *r_key)?;

        for (_i, l_key) in l_keys_vec.iter().enumerate() {
            let l_key_as_string = js_value_to_string(cx, *l_key)?;
            if l_key_as_string.strict_equals(cx, key_as_string) {
                has_key = true;
            }
        }

        if has_key {
            let r_val: Handle<JsValue> = r.get(cx, key_as_string)?;
            let l_val: Handle<JsValue> = l.get(cx, key_as_string)?;

            let diff: Handle<JsValue> = updated_diff(cx, l_val, r_val)?;

            let diff_is_object_and_empty = is_object_and_empty(cx, diff);
            let is_l_val_empty_object = is_object_and_empty(cx, l_val);
            let is_r_val_empty_object = is_object_and_empty(cx, r_val);

            if diff_is_object_and_empty && is_date(cx, diff) == false && (is_l_val_empty_object == true || is_r_val_empty_object == false) {
                //
            } else {
                let length_as_string = cx.string("length");
                if key_as_string.strict_equals(cx, length_as_string) == false {
                    acc.set(cx, key_as_string, diff)?;
                }
            }
        }
    }

    Ok(acc.upcast())
}

fn bind_added_diff(mut cx: FunctionContext) -> JsResult<JsValue> {
    let lhs: Handle<JsValue> = cx.argument::<JsValue>(0)?;
    let rhs: Handle<JsValue> = cx.argument::<JsValue>(1)?;

    added_diff(&mut cx, lhs, rhs)
}

fn bind_deleted_diff(mut cx: FunctionContext) -> JsResult<JsObject> {
    let lhs: Handle<JsValue> = cx.argument::<JsValue>(0)?;
    let rhs: Handle<JsValue> = cx.argument::<JsValue>(1)?;

    deleted_diff(&mut cx, lhs, rhs)
}

fn bind_updated_diff(mut cx: FunctionContext) -> JsResult<JsValue> {
    let lhs: Handle<JsValue> = cx.argument::<JsValue>(0)?;
    let rhs: Handle<JsValue> = cx.argument::<JsValue>(1)?;

    let diff = updated_diff(&mut cx, lhs, rhs)?;

    return Ok(diff);
}

fn bind_is_empty(
    mut cx: FunctionContext
) -> JsResult<JsBoolean> {
    let obj = cx.argument::<JsObject>(0)?;
    let keys: Handle<JsArray> = get_object_keys(&mut cx, obj)?;
    let is_obj_empty: bool = keys.is_empty(&mut cx);
    Ok(cx.boolean(is_obj_empty))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("isEmpty", bind_is_empty)?;
    cx.export_function("addedDiff", bind_added_diff)?;
    cx.export_function("deleted", bind_deleted_diff)?;
    cx.export_function("updated", bind_updated_diff)?;
    Ok(())
}
