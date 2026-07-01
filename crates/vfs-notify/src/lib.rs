//! An implementation of `loader::Handle`, based on `walkdir`.
#![expect(clippy::disallowed_names, reason = "vfs-notify is vendored in")]
use std::{
    fs,
    path::{Component, Path},
    sync::atomic::AtomicUsize,
};

use crossbeam_channel::{Receiver, Sender, select, unbounded};
use paths::{AbsPath, AbsPathBuf, Utf8PathBuf};
use rayon::iter::{IndexedParallelIterator as _, IntoParallelIterator as _, ParallelIterator as _};
use rustc_hash::FxHashSet;
use vfs::loader::{self, LoadingProgress};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct NotifyHandle {
    // Relative order of fields below is significant.
    sender: Sender<Message>,
    _thread: stdx::thread::JoinHandle,
}

#[derive(Debug)]
enum Message {
    Config(loader::Config),
    Invalidate(AbsPathBuf),
}

impl loader::Handle for NotifyHandle {
    fn spawn(sender: loader::Sender) -> Self {
        let actor = NotifyActor::new(sender);
        let (sender, receiver) = unbounded::<Message>();
        let thread = stdx::thread::Builder::new(stdx::thread::ThreadIntent::Worker, "VfsLoader")
            .spawn(move || actor.run(&receiver))
            .expect("failed to spawn thread");

        Self {
            sender,
            _thread: thread,
        }
    }

    fn set_config(
        &mut self,
        config: loader::Config,
    ) {
        self.sender.send(Message::Config(config)).unwrap();
    }

    fn invalidate(
        &mut self,
        path: AbsPathBuf,
    ) {
        self.sender.send(Message::Invalidate(path)).unwrap();
    }

    fn load_sync(
        &mut self,
        path: &AbsPath,
    ) -> Option<Vec<u8>> {
        read(path)
    }
}

struct NotifyActor {
    sender: loader::Sender,
}

impl NotifyActor {
    const fn new(sender: loader::Sender) -> Self {
        Self { sender }
    }

    fn run(
        mut self,
        inbox: &Receiver<Message>,
    ) {
        while let Ok(message) = inbox.recv() {
            tracing::debug!(?message, "vfs-notify message");
            match message {
                Message::Config(config) => {
                    let config_version = config.version;

                    let n_total = config.load.len();

                    self.send(loader::Message::Progress {
                        n_total,
                        n_done: LoadingProgress::Started,
                        config_version,
                        directory: None,
                    });

                    let processed = AtomicUsize::new(0);

                    config.load.into_par_iter().for_each(|entry| {
                        let files = Self::load_entry(entry, |file| {
                            self.send(loader::Message::Progress {
                                n_total,
                                n_done: LoadingProgress::Progress(
                                    processed.load(std::sync::atomic::Ordering::Relaxed),
                                ),
                                directory: Some(file),
                                config_version,
                            });
                        });
                        self.send(loader::Message::Loaded { files });
                        self.send(loader::Message::Progress {
                            n_total,
                            n_done: LoadingProgress::Progress(
                                processed.fetch_add(1, std::sync::atomic::Ordering::AcqRel) + 1,
                            ),
                            config_version,
                            directory: None,
                        });
                    });

                    self.send(loader::Message::Progress {
                        n_total,
                        n_done: LoadingProgress::Finished,
                        config_version,
                        directory: None,
                    });
                },
                Message::Invalidate(path) => {
                    let contents = read(path.as_path());
                    let files = vec![(path, contents)];
                    self.send(loader::Message::Changed { files });
                },
            }
        }
    }

    fn load_entry(
        entry: loader::Entry,
        send_message: impl Fn(AbsPathBuf),
    ) -> Vec<(AbsPathBuf, Option<Vec<u8>>)> {
        match entry {
            loader::Entry::Files(files) => files
                .into_iter()
                .map(|file| {
                    let contents = read(file.as_path());
                    (file, contents)
                })
                .collect::<Vec<_>>(),
            loader::Entry::Directories(directories) => {
                let mut result = Vec::new();

                for root in &directories.include {
                    send_message(root.clone());
                    let walkdir = WalkDir::new(root)
                        .follow_links(true)
                        .into_iter()
                        .filter_entry(|entry| {
                            if !entry.file_type().is_dir() {
                                return true;
                            }
                            let path = entry.path();

                            if path_might_be_cyclic(path) {
                                return false;
                            }

                            // We want to filter out subdirectories that are roots themselves, because they will be visited separately.
                            directories.exclude.iter().all(|it| it != path)
                                && (root == path || directories.include.iter().all(|it| it != path))
                        });

                    let files = walkdir.filter_map(Result::ok).filter_map(|entry| {
                        let depth = entry.depth();
                        let is_dir = entry.file_type().is_dir();
                        let is_file = entry.file_type().is_file();
                        let abs_path = AbsPathBuf::try_from(
                            Utf8PathBuf::from_path_buf(entry.into_path()).ok()?,
                        )
                        .ok()?;
                        if depth < 2 && is_dir {
                            send_message(abs_path.clone());
                        }
                        if !is_file {
                            return None;
                        }
                        let ext = abs_path.extension().unwrap_or_default();
                        if directories.extensions.iter().all(|it| it.as_str() != ext) {
                            return None;
                        }
                        Some(abs_path)
                    });

                    result.extend(files.map(|file| {
                        let contents = read(file.as_path());
                        (file, contents)
                    }));
                }
                result
            },
        }
    }

    #[track_caller]
    fn send(
        &self,
        message: loader::Message,
    ) {
        self.sender.send(message).unwrap();
    }
}

fn read(path: &AbsPath) -> Option<Vec<u8>> {
    std::fs::read(path).ok()
}

/// Is `path` a symlink to a parent directory?
///
/// Including this path is guaranteed to cause an infinite loop. This
/// heuristic is not sufficient to catch all symlink cycles (it's
/// possible to construct cycle using two or more symlinks), but it
/// catches common cases.
fn path_might_be_cyclic(path: &Path) -> bool {
    let Ok(destination) = std::fs::read_link(path) else {
        return false;
    };

    // If the symlink is of the form "../..", it's a parent symlink.
    let is_relative_parent = destination
        .components()
        .all(|component| matches!(component, Component::CurDir | Component::ParentDir));

    is_relative_parent || path.starts_with(destination)
}
