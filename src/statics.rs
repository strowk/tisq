use once_cell::sync::Lazy;
use std::fs::File;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Mutex;

use crate::files;

pub(crate) static QUIT_CHANNEL: Lazy<Mutex<(Sender<String>, Receiver<String>)>> =
    Lazy::new(|| Mutex::new(mpsc::channel()));

pub(crate) static DEBUG_LOG: AtomicBool = AtomicBool::new(false);

pub(crate) static FILES_ROOT: Lazy<eyre::Result<PathBuf>> = Lazy::new(|| files::open_tisq_root());

pub(crate) static LOG_FILE: Lazy<Option<File>> = Lazy::new(|| {
    File::create(if DEBUG_LOG.load(std::sync::atomic::Ordering::Relaxed) {
        FILES_ROOT.as_ref().unwrap().join("tisq-debug.log")
    } else {
        FILES_ROOT.as_ref().unwrap().join("tisq-errors.log")
    })
    .map_err(|_| {
        QUIT_CHANNEL
            .lock()
            .unwrap()
            .0
            .send("Failed to create log file `tisq.log` - exited abnormally.".to_string())
            .unwrap();
    })
    .ok()
});
