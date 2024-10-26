use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, Sample, SizedSample,
};

use jni::{
    signature::ReturnType,
    sys::{jint, jsize, JavaVM},
};
use std::{ffi::c_void, ptr::null_mut};

uniffi::include_scaffolding!("androidcpaltest");

pub type JniGetCreatedJavaVms =
unsafe extern "system" fn(vmBuf: *mut *mut JavaVM, bufLen: jsize, nVMs: *mut jsize) -> jint;
pub const JNI_GET_JAVA_VMS_NAME: &[u8] = b"JNI_GetCreatedJavaVMs";

#[no_mangle]
pub unsafe extern "system" fn initialize_android_context() {
    let lib = libloading::os::unix::Library::this();
    let get_created_java_vms: JniGetCreatedJavaVms =
        unsafe { *lib.get(JNI_GET_JAVA_VMS_NAME).unwrap() };
    let mut created_java_vms: [*mut JavaVM; 1] = [null_mut() as *mut JavaVM];
    let mut java_vms_count: i32 = 0;
    unsafe {
        get_created_java_vms(created_java_vms.as_mut_ptr(), 1, &mut java_vms_count);
    }
    let jvm_ptr = *created_java_vms.first().unwrap();
    let jvm = unsafe { jni::JavaVM::from_raw(jvm_ptr) }.unwrap();
    let mut env = jvm.get_env().unwrap();

    let activity_thread = env.find_class("android/app/ActivityThread").unwrap();
    let current_activity_thread = env
        .get_static_method_id(
            &activity_thread,
            "currentActivityThread",
            "()Landroid/app/ActivityThread;",
        )
        .unwrap();
    let at = env
        .call_static_method_unchecked(
            &activity_thread,
            current_activity_thread,
            ReturnType::Object,
            &[],
        )
        .unwrap();

    let get_application = env
        .get_method_id(
            activity_thread,
            "getApplication",
            "()Landroid/app/Application;",
        )
        .unwrap();
    let context = env
        .call_method_unchecked(at.l().unwrap(), get_application, ReturnType::Object, &[])
        .unwrap();

    ndk_context::initialize_android_context(
        jvm.get_java_vm_pointer() as *mut c_void,
        context.l().unwrap().to_owned() as *mut c_void,
    );
}

pub fn play() {
    let host = cpal::default_host();

    let device = host.default_output_device().unwrap();
    let config = device.default_output_config().unwrap();
    println!("Default output config: {:?}", config);

    match config.sample_format() {
        cpal::SampleFormat::I8 => run::<i8>(&device, &config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        // cpal::SampleFormat::I24 => run::<I24>(&device, &config.into()),
        cpal::SampleFormat::I32 => run::<i32>(&device, &config.into()),
        // cpal::SampleFormat::I48 => run::<I48>(&device, &config.into()),
        cpal::SampleFormat::I64 => run::<i64>(&device, &config.into()),
        cpal::SampleFormat::U8 => run::<u8>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
        // cpal::SampleFormat::U24 => run::<U24>(&device, &config.into()),
        cpal::SampleFormat::U32 => run::<u32>(&device, &config.into()),
        // cpal::SampleFormat::U48 => run::<U48>(&device, &config.into()),
        cpal::SampleFormat::U64 => run::<u64>(&device, &config.into()),
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        cpal::SampleFormat::F64 => run::<f64>(&device, &config.into()),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    }
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig)
where
    T: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of half amplitude.
    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin() * 0.5
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
        None,
    ).expect("Failed to build output stream");
    stream.play().expect("Failed to play stream");

    std::thread::sleep(std::time::Duration::from_millis(1000));
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
