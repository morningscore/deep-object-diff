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

fn js_value_to_string<'a>(
    cx: &mut impl Context<'a>,
    value: Handle<'a, JsValue>,
) -> JsResult<'a, JsString> {
    value.downcast(cx).or_throw(cx)
}

fn is_object_empty<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> JsResult<'a, JsBoolean> {
    let keys: Handle<JsArray> = get_object_keys(cx, obj)?;
    let is_obj_empty: bool = keys.is_empty(cx);
    Ok(cx.boolean(is_obj_empty))
}

fn added_diff<'a>(
    cx: &mut impl Context<'a>,
    lhs: Handle<'a, JsValue>,
    rhs: Handle<'a, JsValue>,
) -> JsResult<'a, JsObject> {
    if rhs.strict_equals(cx, lhs) {
        return Ok(cx.empty_object());
    }

    if rhs.is_a::<JsObject, _>(cx) == false {
        return Ok(cx.empty_object());
    }

    if lhs.is_a::<JsObject, _>(cx) == false {
        return Ok(cx.empty_object());
    }

    let r: Handle<JsObject> = js_value_to_object(cx, rhs)?;
    let l: Handle<JsObject> = js_value_to_object(cx, lhs)?;

    let r_keys: Handle<JsArray> = get_object_keys(cx, r)?;
    let l_keys: Handle<JsArray> = get_object_keys(cx, l)?;

    let r_keys_vec: Vec<Handle<JsValue>> = r_keys.to_vec(cx)?;
    let l_keys_vec: Vec<Handle<JsValue>> = l_keys.to_vec(cx)?;

    let acc = cx.empty_object();

    for (_i, r_key) in r_keys_vec.iter().enumerate() {
        let mut l_has_r_key: bool = false;
        let key_as_string = js_value_to_string(cx, *r_key)?;
        let r_val: Handle<JsValue> = r.get(cx, key_as_string)?;

        for (_i, l_key) in l_keys_vec.iter().enumerate() {
            let l_key_as_string = js_value_to_string(cx, *l_key)?;
            if l_key_as_string.strict_equals(cx, key_as_string) {
                l_has_r_key = true;
            }
        }

        if l_has_r_key {
            let l_val: Handle<JsValue> = l.get(cx, key_as_string)?;
            let diff: Handle<JsObject> = added_diff(cx, l_val, r_val)?;

            if diff.is_a::<JsObject, _>(cx) == true {
                let is_diff_empty: Handle<JsBoolean> = is_object_empty(cx, diff)?;
                let false_check = cx.boolean(false);
                if is_diff_empty.strict_equals(cx, false_check) {
                    acc.set(cx, key_as_string, diff)?;
                }
            } else {
                acc.set(cx, key_as_string, diff)?;
            }
        } else {
            acc.set(cx, key_as_string, r_val)?;
        }
    }

    Ok(acc)
}

fn bind_added_diff(mut cx: FunctionContext) -> JsResult<JsObject> {
    let lhs: Handle<JsValue> = cx.argument::<JsValue>(0)?;
    let rhs: Handle<JsValue> = cx.argument::<JsValue>(1)?;

    added_diff(&mut cx, lhs, rhs)
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
    Ok(())
}
