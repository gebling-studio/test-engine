use std::{path::PathBuf, sync::OnceLock};

static ROOT_PATH: OnceLock<PathBuf> = OnceLock::new();

pub struct Assets;

impl Assets {
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn init(root_path: impl Into<PathBuf>) {
        use refs::manage::DataManager;

        hreads::assert_main_thread();

        let root_path = root_path.into();

        ROOT_PATH.set(root_path.clone()).expect("Double setting of root path");

        let paths = crate::assets_paths::AssetsPaths::new(root_path);

        crate::window::image::Image::set_root_path(&paths.images);
        crate::audio::Sound::set_root_path(&paths.sounds);
        crate::window::Font::set_root_path(&paths.fonts);
    }

    pub fn path() -> PathBuf {
        ROOT_PATH
            .get()
            .expect("Assets root path is not set yet")
            .as_path()
            .join("assets")
    }
}
