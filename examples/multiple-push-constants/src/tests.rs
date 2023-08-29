use std::sync::Arc;

use super::*;
use wgpu_test::{initialize_test, TestParameters};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[wasm_bindgen_test::wasm_bindgen_test]
fn test_compute_1() {
    initialize_test(
        TestParameters::default()
            .downlevel_flags(wgpu::DownlevelFlags::COMPUTE_SHADERS)
            .limits(wgpu::Limits {
                max_push_constant_size: 256,
                ..wgpu::Limits::downlevel_defaults()
            })
            .features(wgpu::Features::TIMESTAMP_QUERY.union(wgpu::Features::PUSH_CONSTANTS))
            .specific_failure(None, None, Some("V3D"), true),
        |ctx| {
            let input: (u32, u32, &[u32]) = (2, 7, &[1, 2, 3, 4]);

            pollster::block_on(assert_execute_gpu(
                &ctx.device,
                &ctx.queue,
                input,
                &[9, 11, 13, 15],
            ));
        },
    );
}

#[test]
#[wasm_bindgen_test::wasm_bindgen_test]
fn test_compute_2() {
    initialize_test(
        TestParameters::default()
            .downlevel_flags(wgpu::DownlevelFlags::COMPUTE_SHADERS)
            .limits(wgpu::Limits {
                max_push_constant_size: 256,
                ..wgpu::Limits::downlevel_defaults()
            })
            .features(wgpu::Features::TIMESTAMP_QUERY.union(wgpu::Features::PUSH_CONSTANTS))
            .specific_failure(None, None, Some("V3D"), true),
        |ctx| {
            let input: (u32, u32, &[u32]) = (5, 3, &[5, 23, 10, 9]);

            pollster::block_on(assert_execute_gpu(
                &ctx.device,
                &ctx.queue,
                input,
                &[28, 118, 53, 48],
            ));
        },
    );
}

#[test]
#[wasm_bindgen_test::wasm_bindgen_test]
fn test_compute_overflow() {
    initialize_test(
        TestParameters::default()
            .downlevel_flags(wgpu::DownlevelFlags::COMPUTE_SHADERS)
            .limits(wgpu::Limits {
                max_push_constant_size: 256,
                ..wgpu::Limits::downlevel_defaults()
            })
            .features(wgpu::Features::TIMESTAMP_QUERY.union(wgpu::Features::PUSH_CONSTANTS))
            .specific_failure(None, None, Some("V3D"), true),
        |ctx| {
            let input: (u32, u32, &[u32]) = (500, 5, &[77031, 837799, 8400511, 63728127]);
            pollster::block_on(assert_execute_gpu(
                &ctx.device,
                &ctx.queue,
                input,
                &[38515505, 418899505, 4200255505, 1799292433],
            ));
        },
    );
}

#[test]
// Wasm doesn't support threads
fn test_multithreaded_compute() {
    initialize_test(
        TestParameters::default()
            .downlevel_flags(wgpu::DownlevelFlags::COMPUTE_SHADERS)
            .limits(wgpu::Limits {
                max_push_constant_size: 256,
                ..wgpu::Limits::downlevel_defaults()
            })
            .features(wgpu::Features::TIMESTAMP_QUERY.union(wgpu::Features::PUSH_CONSTANTS))
            .specific_failure(None, None, Some("V3D"), true)
            // https://github.com/gfx-rs/wgpu/issues/3944
            .specific_failure(
                Some(wgpu::Backends::VULKAN),
                None,
                Some("swiftshader"),
                true,
            )
            // https://github.com/gfx-rs/wgpu/issues/3250
            .specific_failure(Some(wgpu::Backends::GL), None, Some("llvmpipe"), true),
        |ctx| {
            use std::{sync::mpsc, thread, time::Duration};

            let ctx = Arc::new(ctx);

            let thread_count = 8;

            let (tx, rx) = mpsc::channel();
            for _ in 0..thread_count {
                let tx = tx.clone();
                let ctx = Arc::clone(&ctx);
                thread::spawn(move || {
                    let input: (u32, u32, &[u32]) = (3, 5, &[100, 100, 100]);
                    pollster::block_on(assert_execute_gpu(
                        &ctx.device,
                        &ctx.queue,
                        input,
                        &[305, 305, 305],
                    ));
                    tx.send(true).unwrap();
                });
            }

            for _ in 0..thread_count {
                rx.recv_timeout(Duration::from_secs(10))
                    .expect("A thread never completed.");
            }
        },
    );
}

async fn assert_execute_gpu(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    input: (u32, u32, &[u32]),
    expected: &[u32],
) {
    if let Some(produced) = execute_gpu_inner(device, queue, input.0, input.1, input.2).await {
        assert_eq!(produced, expected);
    }
}
