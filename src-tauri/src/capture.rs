use crate::error::{LumeError, Result};
use scap::capturer::{Capturer, Options};
use scap::frame::FrameType;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use tauri::{AppHandle, Emitter};

/// A running capture: the stop flag the worker polls, and its thread handle
/// (which yields the frame count on join).
pub struct CaptureHandle {
    stop: Arc<AtomicBool>,
    thread: JoinHandle<u64>,
}

/// Managed state holding the current capture, if any.
#[derive(Default)]
pub struct CaptureState(pub Mutex<Option<CaptureHandle>>);

/// Start capturing the main display. The scap capturer is built and owned
/// entirely inside the worker thread (the macOS engine isn't `Send`); the
/// thread reports whether it started so permission/support errors surface here.
#[tauri::command]
pub fn start_capture(app: AppHandle, state: tauri::State<'_, CaptureState>) -> Result<()> {
    let mut slot = state.0.lock().expect("capture state poisoned");
    if slot.is_some() {
        return Err(LumeError::Capture("already recording".into()));
    }

    let stop = Arc::new(AtomicBool::new(false));
    let stop_worker = stop.clone();
    let (ready_tx, ready_rx) = mpsc::channel::<std::result::Result<(), String>>();

    let thread = std::thread::spawn(move || {
        let options = Options {
            fps: 30,
            show_cursor: true,
            output_type: FrameType::BGRAFrame,
            ..Default::default()
        };

        let mut capturer = match Capturer::build(options) {
            Ok(capturer) => {
                let _ = ready_tx.send(Ok(()));
                capturer
            }
            Err(err) => {
                let _ = ready_tx.send(Err(err.to_string()));
                return 0;
            }
        };

        capturer.start_capture();
        let mut frames: u64 = 0;
        while !stop_worker.load(Ordering::Relaxed) {
            match capturer.get_next_frame() {
                Ok(_frame) => frames += 1,
                Err(_) => break, // engine stopped / channel closed
            }
        }
        capturer.stop_capture();
        frames
    });

    match ready_rx.recv() {
        Ok(Ok(())) => {}
        Ok(Err(message)) => return Err(LumeError::Capture(message)),
        Err(_) => return Err(LumeError::Capture("capture thread exited early".into())),
    }

    *slot = Some(CaptureHandle { stop, thread });
    let _ = app.emit("capture-status", "recording");
    log::info!("capture started");
    Ok(())
}

/// Stop the current capture and return the number of frames it produced.
#[tauri::command]
pub fn stop_capture(app: AppHandle, state: tauri::State<'_, CaptureState>) -> Result<u64> {
    let handle = state.0.lock().expect("capture state poisoned").take();
    let Some(handle) = handle else {
        return Err(LumeError::Capture("not recording".into()));
    };

    handle.stop.store(true, Ordering::Relaxed);
    let frames = handle.thread.join().unwrap_or(0);

    let _ = app.emit("capture-status", "idle");
    log::info!("capture stopped after {frames} frames");
    Ok(frames)
}
