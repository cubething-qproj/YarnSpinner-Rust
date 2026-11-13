use crate::prelude::*;
use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use yarnspinner::compiler::Project;

/// A .yarnproject file.
#[derive(Debug, Clone, PartialEq, Reflect, Asset, Serialize, Deserialize, Deref, DerefMut)]
#[reflect(Debug, PartialEq, Serialize, Deserialize)]
pub struct YarnProjectFile(pub(crate) Project);

#[doc = "[AssetLoader] for a [Project]."]
#[derive(Debug, Default, TypePath)]
pub struct YarnProjectFileLoader;

impl AssetLoader for YarnProjectFileLoader {
    type Asset = YarnProjectFile;
    type Settings = ();
    type Error = anyhow::Error;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let file: Project = serde_json::from_slice(&bytes)?;
        let path = load_context.path().path();
        let file = file.with_path(path.to_path_buf());
        Ok(YarnProjectFile(file))
    }
    fn extensions(&self) -> &[&str] {
        &["yarnproject"]
    }
}

pub fn plugin(app: &mut App) {
    app.init_asset::<YarnProjectFile>()
        .register_asset_reflect::<YarnProjectFile>()
        .init_asset_loader::<YarnProjectFileLoader>();
}
