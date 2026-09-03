use crate::global_state::GlobalState;
use base_db::{change::Change as BaseDbChange, input::PackageOrigin};
use edition::Edition;
use project_model::{ManifestPath, PackageKey, WeslPackage};
use vfs::{AbsPathBuf, VfsPath, VirtualPath};
use wgsl_std::StdLibrary;

impl GlobalState {
    pub(crate) fn load_builtin_package(&self) {
        // I need to add them to the vfs here, so that self.source_root_config.partition(vfs) continues to work.
        let mut guard = self.vfs.write();
        let (vfs, line_endings_map) = &mut *guard;
        let std_library = StdLibrary::new();
        for file in std_library.files {
            vfs.set_file_contents(
                VfsPath::new_virtual_path(file.path),
                Some(file.contents.to_vec()),
            );
        }
        vfs.set_file_contents(
            VfsPath::new_virtual_path(std_library.manifest.path.clone()),
            Some(std_library.manifest.contents.to_vec()),
        );
        std::mem::drop(guard);

        let mut packages = self.packages.write();
        packages.set(
            PackageKey::VirtualManifest(VirtualPath::new(std_library.manifest.path.clone())),
            WeslPackage {
                manifest: VfsPath::new_virtual_path(std_library.manifest.path),
                display_name: Some("std".to_owned()),
                root: VfsPath::new_virtual_path("/std".to_owned()),
                origin: PackageOrigin::Language,
                dependencies: Vec::new(),
                edition: Edition::LATEST,
            },
        );
        std::mem::drop(packages);
    }
}
