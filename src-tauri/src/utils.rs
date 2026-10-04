#[tauri::command]
pub async fn lock_orientation(
    window: tauri::WebviewWindow,
    orientation: String,
) -> Result<bool, String> {
    let _value: i32 = match orientation.as_str() {
        "portrait" => 1,
        "landscape" => 0,
        "sensor_portrait" => 7,
        "sensor_landscape" => 6,
        "unlock" => -1,
        other => return Err(format!("orientation inconnue: {other}")),
    };

    #[cfg(target_os = "android")]
    {
        use jni::objects::JValue;

        window
            .with_webview(move |webview| {
                webview.jni_handle().exec(move |env, activity, _webview| {
                    let res = env.call_method(
                        activity,
                        "setRequestedOrientation",
                        "(I)V",
                        &[JValue::Int(_value)],
                    );

                    if res.is_err() || env.exception_check().unwrap_or(false) {
                        let _ = env.exception_describe();
                        let _ = env.exception_clear();
                        eprintln!("setRequestedOrientation a échoué");
                    }
                });
            })
            .map_err(|e| e.to_string())?;
    }
    return Ok(true);
}
