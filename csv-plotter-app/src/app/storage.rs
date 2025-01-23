use std::{collections::HashMap, path::PathBuf};

use app_core::storage::Storage;
use serde::{Deserialize, Serialize};

use crate::EguiApp;

use super::{
    components::{File, FileID, FileProperties, Group},
    DynRequestSender, FileHandler,
};

// Currently not used, since the only backend state to safe right now is the
// search path, which is also mirrored in the frontend (app.search).
#[derive(Clone, Serialize, Deserialize)]
struct BackendStorage {}

#[derive(Serialize, Deserialize)]
struct FrontendStorage {
    search_path: PathBuf,
    groups: [Option<Group>; 10],
    registry: HashMap<FileID, FileStorage>,
    next_id: FileID,
}

pub fn save_json(app: &EguiApp) -> Result<(), String> {
    let backend_storage = BackendStorage {};

    let frontend_storage = FrontendStorage {
        search_path: app.search.get_search_path().to_path_buf(),
        groups: app.file_handler.groups.clone(),
        registry: app
            .file_handler
            .registry
            .iter()
            .map(|(fid, file)| {
                (
                    *fid,
                    FileStorage {
                        path: file.path.clone(),
                        properties: file.properties.clone(),
                    },
                )
            })
            .collect(),
        next_id: app.file_handler.current_id(),
    };
    let storage = Storage::new(backend_storage, frontend_storage);
    storage.save_json()
}

pub fn load_json(app: &mut EguiApp) -> Result<(), String> {
    let Storage::<BackendStorage, FrontendStorage> {
        backend_storage: _,
        frontend_storage,
    } = Storage::from_json()?;

    app.search.set_search_path(&frontend_storage.search_path);
    app.file_handler = frontend_storage.into_file_handler(&mut app.request_tx);
    Ok(())
}

// Serializing the files is a special case, because we do not want to store the
// entire data contained in the csv files (plus it is non-trivial to do this).
#[derive(Serialize, Deserialize)]
struct FileStorage {
    path: PathBuf,
    properties: FileProperties,
}

impl FrontendStorage {
    fn into_file_handler(self, request_tx: &mut DynRequestSender) -> FileHandler {
        let groups = self.groups;
        let registry = self
            .registry
            .into_iter()
            .map(|(fid, file_storage)| {
                (
                    fid,
                    File::new(file_storage.path, file_storage.properties, request_tx),
                )
            })
            .collect();

        FileHandler::new(groups, registry, self.next_id)
    }
}