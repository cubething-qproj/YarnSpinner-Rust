use crate::prelude::*;
use bevy::prelude::*;

use crate::plugin::AssetRoot;
use std::path::PathBuf;

/// A source for the YarnProjects.
#[derive(Debug, Reflect, PartialEq, Eq, Hash, Clone)]
pub enum YarnProjectSource {
    /// A [`YarnProjectFile`] that is already present in the asset server.
    Handle(Handle<YarnProjectFile>),
    /// A [`YarnProjectFile`] in the `assets` folder which will be loaded into the [`AssetServer`].
    File(PathBuf),
    // todo: support folder when implementing multiple project files
}

impl From<Handle<YarnProjectFile>> for YarnProjectSource {
    fn from(value: Handle<YarnProjectFile>) -> Self {
        Self::Handle(value)
    }
}

impl YarnProjectSource {
    /// Convenience function to create a [`YarnProjectSource::File`]
    pub fn file(path: impl Into<PathBuf>) -> Self {
        Self::File(path.into())
    }

    pub(crate) fn load(
        &self,
        asset_server: &AssetServer,
        #[allow(unused_variables, reason = "conditional compilation")] asset_root: &AssetRoot,
    ) -> Handle<YarnProjectFile> {
        match self {
            YarnProjectSource::Handle(handle) => handle.clone(),
            YarnProjectSource::File(path) => asset_server.load(path.to_string_lossy().into_owned()),
        }
    }
}
