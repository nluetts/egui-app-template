mod logic;
mod ui;

use std::{collections::HashSet, path::PathBuf, thread::JoinHandle};

use app_core::frontend::UIParameter;

use crate::app::DynRequestSender;

pub struct Search {
    #[allow(clippy::complexity)]
    /// PathBuf is the path of the matched file, HashSet<usize> the indices
    /// of the matched substrings, and Option<usize> the group where the file
    /// should be inserted.
    matched_paths: UIParameter<Vec<(PathBuf, HashSet<usize>, Option<usize>)>>,
    popup_shown: bool,
    search_path: UIParameter<PathBuf>,
    search_query: String,
    awaiting_search_path_selection: Option<JoinHandle<Option<PathBuf>>>,
    request_tx: DynRequestSender,
}

impl Search {
    pub fn new(request_tx: DynRequestSender) -> Self {
        Self {
            matched_paths: Default::default(),
            popup_shown: Default::default(),
            search_path: Default::default(),
            search_query: Default::default(),
            awaiting_search_path_selection: Default::default(),
            request_tx,
        }
    }
}
