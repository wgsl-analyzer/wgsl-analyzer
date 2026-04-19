use base_db::{ExtraPackageData, Package, input::PackageData};
use ide_db::RootDatabase;
use itertools::Itertools as _;
use stdx::format_to;
use vfs::FileId;

// Feature: Status
//
// Shows internal statistic about memory usage of rust-analyzer.
//
// | Editor  | Action Name |
// |---------|-------------|
// | VS Code | **rust-analyzer: Status** |
//
// ![Status](https://user-images.githubusercontent.com/48062697/113065584-05f34500-91b1-11eb-98cc-5c196f76be7f.gif)
pub(crate) fn status(
    database: &RootDatabase,
    file_id: Option<FileId>,
) -> String {
    let mut buffer = String::new();

    // format_to!(buf, "{}\n", collect_query(CompressedFileTextQuery.in_db(db)));
    // format_to!(buf, "{}\n", collect_query(ParseQuery.in_db(db)));
    // format_to!(buf, "{}\n", collect_query(ParseMacroExpansionQuery.in_db(db)));
    // format_to!(buf, "{}\n", collect_query(LibrarySymbolsQuery.in_db(db)));
    // format_to!(buf, "{}\n", collect_query(ModuleSymbolsQuery.in_db(db)));
    // format_to!(buf, "{} in total\n", memory_usage());

    // format_to!(buf, "\nDebug info:\n");
    // format_to!(buf, "{}\n", collect_query(AttrsQuery.in_db(db)));
    // format_to!(buf, "{} ast id maps\n", collect_query_count(AstIdMapQuery.in_db(db)));
    // format_to!(buf, "{} block def maps\n", collect_query_count(BlockDefMapQuery.in_db(db)));

    if let Some(file_id) = file_id {
        format_to!(buffer, "\nCrates for file {}:\n", file_id.index());
        let packages: Vec<Package> = Vec::new(); // TODO: populate this
        if packages.is_empty() {
            format_to!(buffer, "Does not belong to any package");
        }
        for package_id in packages {
            let PackageData {
                root_file_id,
                display_name,
                edition,
                dependencies,
                origin,
            } = package_id.data(database);
            // let ExtraPackageData {
            //     version,
            //     display_name,
            // } = package_id.extra_data(database);
            format_to!(
                buffer,
                "Crate: {}\n",
                match display_name {
                    Some(display_name) => format!("{display_name}({package_id:?})"),
                    None => format!("{package_id:?}"),
                }
            );
            format_to!(
                buffer,
                "    Root module file id: {}\n",
                root_file_id.index()
            );
            format_to!(buffer, "    Edition: {}\n", edition);
            // format_to!(
            //     buffer,
            //     "    Version: {}\n",
            //     version.as_deref().unwrap_or("n/a")
            // );
            format_to!(buffer, "    Origin: {:?}\n", origin);
            let deps = dependencies
                .iter()
                .map(|dep| format!("{}={:?}", dep.name, dep.package_id))
                .format(", ");
            format_to!(buffer, "    Dependencies: {}\n", deps);
        }
    }

    buffer.trim().to_owned()
}
