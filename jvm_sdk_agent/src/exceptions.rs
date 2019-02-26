use rollbar_jvm::env::JvmTiEnv;
use rollbar_jvm::errors::*;
use rollbar_jvm::exceptions::build_stack_trace_frames;
use rollbar_jvm::jni::JniEnv;

use rollbar_jvm::jvmti::{jobject, jthread};

pub fn inner_callback(
    mut jvmti_env: JvmTiEnv,
    mut jni_env: JniEnv,
    thread: jthread,
    exception: jobject,
) -> Result<()> {
    trace!("on_exception called");
    let class = jni_env.find_class("com/rollbar/jvmti/ThrowableCache")?;

    let should_cache_method =
        jni_env.get_static_method_id(class, "shouldCacheThrowable", "(Ljava/lang/Throwable;I)Z")?;

    let num_frames = jvmti_env.get_frame_count(thread)?;

    let shouldCache =
        jni_env.call_static_LI_Z_method(class, should_cache_method, exception, num_frames)?;

    if !shouldCache {
        return Ok(());
    }

    let cache_add_method = jni_env.get_static_method_id(
        class,
        "add",
        "(Ljava/lang/Throwable;[Lcom/rollbar/jvmti/CacheFrame;)V",
    )?;

    let start_depth = 0;
    let frames = build_stack_trace_frames(jvmti_env, jni_env, thread, start_depth, num_frames)?;

    jni_env.call_static_LAL_V_method(class, cache_add_method, exception, frames)?;
    trace!("on_exception exit");
    Ok(())
}
