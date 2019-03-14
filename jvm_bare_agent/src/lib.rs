#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate log;

mod exceptions;
mod rollbar;

use crate::rollbar::Rollbar;
use lazy_static::lazy_static;
use rollbar_jvm::env::JvmTiEnv;
use rollbar_jvm::jni::JniEnv;
use rollbar_jvm::jvmti::*;
use rollbar_rust::Configuration;
use std::env;
use std::os::raw::{c_char, c_void};
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
use std::sync::Once;

static INIT_SUCCESS: AtomicBool = ATOMIC_BOOL_INIT;

static mut CONFIG: Option<Configuration> = None;
static INIT: Once = Once::new();

const ACCESS_TOKEN_KEY: &str = "ROLLBAR_TOKEN";

lazy_static! {
    static ref ROLLBAR: Rollbar = build_client();
}

fn build_client() -> Rollbar {
    let config;
    unsafe {
        config = CONFIG.take();
    }
    Rollbar::new(config.expect("config should be initialized before building client"))
}

fn initialize_configuration() -> bool {
    unsafe {
        INIT.call_once(|| {
            debug!("Loading configuration");
            match Rollbar::configuration_from_file("rollbar.conf") {
                Ok(conf) => {
                    CONFIG = Some(conf);
                }
                Err(err) => {
                    debug!("Error loading configuration: {}", err);
                    match env::var(ACCESS_TOKEN_KEY) {
                        Ok(token) => {
                            let mut conf = Configuration::default();
                            conf.access_token = Some(token);
                            CONFIG = Some(conf);
                        }
                        Err(e) => {
                            debug!("Error loading {}: {}", ACCESS_TOKEN_KEY, e);
                        }
                    }
                }
            }
        });
        CONFIG.is_some()
    }
}

#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn Agent_OnLoad(
    vm: *mut JavaVM,
    options: *mut c_char,
    reserved: *mut c_void,
) -> jint {
    pretty_env_logger::init_custom_env("ROLLBAR_LOG");
    info!("Agent load begin");
    if let Err(e) = onload(vm) {
        return e;
    }
    if !initialize_configuration() {
        return jvmtiError_JVMTI_ERROR_INTERNAL as i32;
    }
    info!("Agent load complete success");
    INIT_SUCCESS.store(true, Ordering::Relaxed);
    0
}

#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn Agent_OnUnload(vm: *mut JavaVM) {
    info!("Agent shutdown begin");
    ROLLBAR.shutdown();
    info!("Agent shutdown success");
}

fn onload(vm: *mut JavaVM) -> Result<(), jint> {
    let mut jvmti_env;
    unsafe {
        jvmti_env = JvmTiEnv::maybe_new(vm)?;
    }
    jvmti_env.enable_capabilities(false)?;
    jvmti_env.set_exception_handler(c_on_exception)?;
    Ok(())
}

fn on_exception(
    jvmti_env: JvmTiEnv,
    jni_env: JniEnv,
    thread: jthread,
    method: jmethodID,
    location: jlocation,
    exception: jobject,
    catch_method: jmethodID,
    catch_location: jlocation,
) {
    if let Err(e) = exceptions::callback(
        &ROLLBAR,
        jvmti_env,
        jni_env,
        thread,
        method,
        location,
        exception,
        catch_method,
        catch_location,
    ) {
        debug!("{}", e);
    }
}

#[allow(unused_variables)]
unsafe extern "C" fn c_on_exception(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    thread: jthread,
    method: jmethodID,
    location: jlocation,
    exception: jobject,
    catch_method: jmethodID,
    catch_location: jlocation,
) {
    if INIT_SUCCESS.load(Ordering::Relaxed) {
        let jvmti_env = JvmTiEnv::wrap(jvmti_env);
        on_exception(
            jvmti_env,
            JniEnv::new(jni_env),
            thread,
            method,
            location,
            exception,
            catch_method,
            catch_location,
        );
    }
}
