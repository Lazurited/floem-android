#[cfg(not(any(target_os = "android", target_os = "ios")))]
use std::path::PathBuf;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use floem_reactive::Scope;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use crate::ext_event::create_ext_action;

use super::file::{FileDialogOptions, FileInfo};

/// Open a file using the system file dialog
pub fn open_file(
    options: FileDialogOptions,
    file_info_action: impl Fn(Option<FileInfo>) + 'static,
) {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let send = create_ext_action(
            Scope::new(),
            move |(path, paths): (Option<PathBuf>, Option<Vec<PathBuf>>)| {
                if paths.is_some() {
                    file_info_action(paths.map(|paths| FileInfo {
                        path: paths,
                        format: None,
                    }))
                } else {
                    file_info_action(path.map(|path| FileInfo {
                        path: vec![path],
                        format: None,
                    }))
                }
            },
        );
        std::thread::spawn(move || {
            let mut dialog = rfd::FileDialog::new();
            if let Some(path) = options.starting_directory.as_ref() {
                dialog = dialog.set_directory(path);
            }
            if let Some(title) = options.title.as_ref() {
                dialog = dialog.set_title(title);
            }
            if let Some(allowed_types) = options.allowed_types.as_ref() {
                dialog = allowed_types.iter().fold(dialog, |dialog, filter| {
                    dialog.add_filter(filter.name, filter.extensions)
                });
            }

            if options.select_directories && options.multi_selection {
                send((None, dialog.pick_folders()));
            } else if options.select_directories && !options.multi_selection {
                send((dialog.pick_folder(), None));
            } else if !options.select_directories && options.multi_selection {
                send((None, dialog.pick_files()));
            } else {
                send((dialog.pick_file(), None));
            }
        });
    }

    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let _ = options;
        file_info_action(None);
    }
}

/// Open a system file save dialog
pub fn save_as(options: FileDialogOptions, file_info_action: impl Fn(Option<FileInfo>) + 'static) {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let send = create_ext_action(Scope::new(), move |path: Option<PathBuf>| {
            file_info_action(path.map(|path| FileInfo {
                path: vec![path],
                format: None,
            }))
        });
        std::thread::spawn(move || {
            let mut dialog = rfd::FileDialog::new();
            if let Some(path) = options.starting_directory.as_ref() {
                dialog = dialog.set_directory(path);
            }
            if let Some(name) = options.default_name.as_ref() {
                dialog = dialog.set_file_name(name);
            }
            if let Some(title) = options.title.as_ref() {
                dialog = dialog.set_title(title);
            }
            let path = dialog.save_file();
            send(path);
        });
    }

    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let _ = options;
        file_info_action(None);
    }
}
