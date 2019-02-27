use crate::rollbar::Rollbar;
use lazy_static::lazy_static;
use regex::RegexSet;
use rollbar_jvm::env::JvmTiEnv;
use rollbar_jvm::errors::*;
use rollbar_jvm::jni::JniEnv;
use rollbar_jvm::jvmti::{jlocation, jmethodID, jobject, jthread};
use rollbar_rust::types::*;
use std::sync::atomic::{AtomicPtr, Ordering};

static LAST_EXCEPTION: AtomicPtr<rollbar_jvm::jvmti::_jobject> =
    AtomicPtr::new(std::ptr::null_mut());

pub fn callback(
    rollbar: &Rollbar,
    _jvmti_env: JvmTiEnv,
    mut jni_env: JniEnv,
    _thread: jthread,
    _method: jmethodID,
    _location: jlocation,
    exception: jobject,
    _catch_method: jmethodID,
    _catch_location: jlocation,
) -> Result<()> {
    trace!("on_exception called");
    if LAST_EXCEPTION.swap(exception, Ordering::SeqCst) == exception {
        trace!("Ignoring same exception object");
        return Ok(());
    }
    let exc = jni_env.get_exception_info(exception)?;
    if should_ignore(exc.class.as_ref()) {
        return Ok(());
    }
    debug!("Report: {}", exc.class);
    let mut frames = jni_env.get_stack_trace(exception)?;
    frames.reverse();
    let trace = Trace::builder().frames(frames).exception(exc).build();
    let body = Body::builder().trace(trace).build();
    let data = Data::builder()
        .body(body)
        .language("java")
        .level(Level::Error);
    rollbar.send(data);
    Ok(())
}

#[allow(clippy::trivial_regex)]
fn should_ignore(class: &str) -> bool {
    lazy_static! {
        static ref RE: RegexSet = RegexSet::new(&[
            r"^com\.sun\.org",
            r"^javax\.naming\.",
            r"^java\.io\.EOFException",
            r"^java\.io\.FileNotFoundException",
            r"^java\.io\.IOException",
            r"^java\.lang\.ArrayIndexOutOfBoundsException",
            r"^java\.lang\.ClassNotFoundException",
            r"^java\.lang\.IllegalStateException",
            r"^java\.lang\.InterruptedException",
            r"^java\.lang\.NoSuchFieldError",
            r"^java\.lang\.NoSuchFieldException",
            r"^java\.lang\.NoSuchMethodException",
            r"^java\.net\.MalformedURLException",
            r"^java\.net\.SocketException",
            r"^java\.security\.cert\.CertificateParsingException",
            r"^java\.security\.PrivilegedActionException",
            r"^java\.security\.SignatureException",
            r"^java\.util\.zip\.ZipException",
            r"^javax\.crypto\.BadPaddingException",
        ])
        .unwrap();
    }
    RE.is_match(class)
}
/*
    if class.starts_with("com.sun.org") || class.starts_with("javax.naming.") {
        return true;
    }
    match class {
        "java.io.EOFException"
        | "java.io.FileNotFoundException"
        | "java.io.IOException"
        | "java.lang.ArrayIndexOutOfBoundsException"
        | "java.lang.ClassNotFoundException"
        | "java.lang.IllegalStateException"
        | "java.lang.InterruptedException"
        | "java.lang.NoSuchFieldError"
        | "java.lang.NoSuchFieldException"
        | "java.lang.NoSuchMethodException"
        | "java.net.MalformedURLException"
        | "java.net.SocketException"
        | "java.security.cert.CertificateParsingException"
        | "java.security.PrivilegedActionException"
        | "java.security.SignatureException"
        | "java.util.zip.ZipException"
        | "javax.crypto.BadPaddingException" => return true,
        _ => {}
    }
    false
}
*/
