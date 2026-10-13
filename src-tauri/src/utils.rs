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

#[cfg(target_os = "android")]
fn apply_immersive(
    env: &mut jni::JNIEnv,
    activity: &jni::objects::JObject,
    hide: bool,
) -> jni::errors::Result<()> {
    use jni::objects::{JClass, JValue};

    const SYSTEM_BARS: i32 = 7; // STATUS_BARS | NAVIGATION_BARS | CAPTION_BAR
    const BEHAVIOR_TRANSIENT: i32 = 2;

    let window = env
        .call_method(activity, "getWindow", "()Landroid/view/Window;", &[])?
        .l()?;
    let decor = env
        .call_method(&window, "getDecorView", "()Landroid/view/View;", &[])?
        .l()?;

    // Charge WindowCompat via le class loader de l'app (pas FindClass)
    let loader = env
        .call_method(activity, "getClassLoader", "()Ljava/lang/ClassLoader;", &[])?
        .l()?;
    let name = env.new_string("androidx.core.view.WindowCompat")?; // des points, pas des slashes
    let class = JClass::from(
        env.call_method(
            &loader,
            "loadClass",
            "(Ljava/lang/String;)Ljava/lang/Class;",
            &[JValue::Object(&name)],
        )?
        .l()?,
    );

    let controller = env
        .call_static_method(
            &class,
            "getInsetsController",
            "(Landroid/view/Window;Landroid/view/View;)Landroidx/core/view/WindowInsetsControllerCompat;",
            &[JValue::Object(&window), JValue::Object(&decor)],
        )?
        .l()?;

    env.call_method(
        &controller,
        "setSystemBarsBehavior",
        "(I)V",
        &[JValue::Int(BEHAVIOR_TRANSIENT)],
    )?;
    env.call_method(
        &controller,
        if hide { "hide" } else { "show" },
        "(I)V",
        &[JValue::Int(SYSTEM_BARS)],
    )?;

    Ok(())
}

#[tauri::command]
pub fn set_fullscreen(window: tauri::WebviewWindow, hide: bool) -> Result<(), String> {
    #[cfg(target_os = "android")]
    {
        window
            .with_webview(move |webview| {
                webview.jni_handle().exec(move |env, activity, _webview| {
                    if apply_immersive(env, activity, hide).is_err() {
                        let _ = env.exception_describe();
                        let _ = env.exception_clear();
                        eprintln!("set_fullscreen a échoué");
                    }
                });
            })
            .map_err(|e| e.to_string())?;
        return Ok(());
    }

    #[cfg(not(target_os = "android"))]
    {
        let _ = (window, hide);
        Err("Android uniquement".into())
    }
}

#[tauri::command]
pub fn quit(app: tauri::AppHandle) {
    app.exit(0);
}