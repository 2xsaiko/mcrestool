use crate::datasource::DataSource;
use std::path::PathBuf;
use crate::FileType;

pub struct Workspace {
    roots: Vec<WorkspaceRoot>,
}

pub struct WorkspaceRoot {
    name: String,
    ds: DataSource,
    root: FsTreeEntry,
}

pub struct FsTreeEntry {
    path: PathBuf,
    typ: FileType,
    children: Vec<FsTreeEntry>,
}