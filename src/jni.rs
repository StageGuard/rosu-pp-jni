use core::mem::size_of;
use jni::objects::JClass;
use jni::JNIEnv;
use jni::sys::{jbyteArray, jdouble, jint, jlong, jstring};
use bytes::BufMut;
use crate::Beatmap;
use crate::osu::OsuPerformance;

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nCreate(
    env: JNIEnv<'_>, _class: JClass<'_>, path: jstring
) -> jlong {
    let path_jstr = match env.get_string(path.into()) {
        Ok(str) => str,
        Err(_) => return 0
    };
    let path = match path_jstr.to_str() {
        Ok(str) => str,
        Err(_) => return 0
    };
    let beatmap = match Beatmap::from_path(path) {
        Ok(b) => b,
        Err(_) => return 0
    };

    let calc = OsuPerformance::try_new(beatmap);
    let leaked = Box::leak(Box::new(calc));

    leaked as *const _ as i64
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nMods(
    _env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPerformance<'static>, mods: jint
) -> jlong {
    let cloned = calc_ptr.clone();
    let _ = unsafe { Box::from_raw(calc_ptr as *mut OsuPerformance) };

    let calc_new = cloned.mods(mods as u32);

    let leaked = Box::leak(Box::new(calc_new));
    leaked as *const _ as i64
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nCombo(
    _env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPerformance<'static>, combo: jint
) -> jlong {
    let cloned = calc_ptr.clone();
    let _ = unsafe { Box::from_raw(calc_ptr as *mut OsuPerformance) };

    let calc_new = cloned.combo(combo as u32);

    let leaked = Box::leak(Box::new(calc_new));
    leaked as *const _ as i64
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nN300(
    _env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPerformance<'static>, n: jint
) -> jlong {
    let cloned = calc_ptr.clone();
    let _ = unsafe { Box::from_raw(calc_ptr as *mut OsuPerformance) };

    let calc_new = cloned.n300(n as u32);

    let leaked = Box::leak(Box::new(calc_new));
    leaked as *const _ as i64
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nN100(
    _env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPerformance<'static>, n: jint
) -> jlong {
    let cloned = calc_ptr.clone();
    let _ = unsafe { Box::from_raw(calc_ptr as *mut OsuPerformance) };

    let calc_new = cloned.n100(n as u32);

    let leaked = Box::leak(Box::new(calc_new));
    leaked as *const _ as i64
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nN50(
    _env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPerformance<'static>, n: jint
) -> jlong {
    let cloned = calc_ptr.clone();
    let _ = unsafe { Box::from_raw(calc_ptr as *mut OsuPerformance) };

    let calc_new = cloned.n50(n as u32);

    let leaked = Box::leak(Box::new(calc_new));
    leaked as *const _ as i64
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nMisses(
    _env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPerformance<'static>, n: jint
) -> jlong {
    let cloned = calc_ptr.clone();
    let _ = unsafe { Box::from_raw(calc_ptr as *mut OsuPerformance) };

    let calc_new = cloned.misses(n as u32);

    let leaked = Box::leak(Box::new(calc_new));
    leaked as *const _ as i64
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nPassedObjects(
    _env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPerformance<'static>, n: jint
) -> jlong {
    let cloned = calc_ptr.clone();
    let _ = unsafe { Box::from_raw(calc_ptr as *mut OsuPerformance) };

    let calc_new = cloned.passed_objects(n as u32);

    let leaked = Box::leak(Box::new(calc_new));
    leaked as *const _ as i64
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nAccuracy(
    _env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPerformance<'static>, acc: jdouble
) -> jlong {
    let cloned = calc_ptr.clone();
    let _ = unsafe { Box::from_raw(calc_ptr as *mut OsuPerformance) };

    let calc_new = cloned.accuracy(acc as f64);

    let leaked = Box::leak(Box::new(calc_new));
    leaked as *const _ as i64
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nCalculate(
    env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPerformance<'static>
) -> jbyteArray {

    let calc = unsafe { Box::from_raw(calc_ptr) };
    let result = calc.calculate()
        .unwrap_or_default();
    let attributes = result.difficulty;

    let mut buffer: Vec<u8> = Vec::with_capacity(
        size_of::<jlong>() + size_of::<jdouble>() * 13 + size_of::<jint>() * 3
    );

    buffer.put_f64(result.pp);
    buffer.put_f64(result.pp_aim);
    buffer.put_f64(result.pp_speed);
    buffer.put_f64(result.pp_acc);
    buffer.put_f64(result.pp_flashlight);

    buffer.put_f64(attributes.stars);
    buffer.put_f64(attributes.ar);
    buffer.put_f64(attributes.od);
    buffer.put_f64(attributes.hp);
    buffer.put_f64(0.0);
    buffer.put_f64(attributes.aim);
    buffer.put_f64(attributes.slider_factor);
    buffer.put_f64(attributes.speed);
    buffer.put_i32(attributes.max_combo as i32);
    buffer.put_i32(attributes.n_circles as i32);
    buffer.put_i32(attributes.n_sliders as i32);
    buffer.put_i32(attributes.n_spinners as i32);

    env.byte_array_from_slice(buffer.as_slice()).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nReleaseCalculator(
    _env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPerformance<'static>
) {
    let _ = unsafe { Box::from_raw(calc_ptr) };
}