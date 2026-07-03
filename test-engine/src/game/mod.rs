#[cfg(not(any(feature = "2d", feature = "3d")))]
compile_error!("To use the game module you must enable one of '2d' or '3d' features.");

#[cfg(all(feature = "2d", feature = "3d"))]
compile_error!("Features '2d' and '3d' are mutually exclusive and cannot be enabled together.");

mod game;
mod object;
mod primitives;
mod shape;

pub use self::{game::*, object::*, primitives::*, shape::*};
pub use crate::game_drawer::GameDrawer;

#[cfg(test)]
mod test {

    use anyhow::Result;
    use gltf::Gltf;

    #[test]
    #[ignore]
    fn test_model_import() -> Result<()> {
        let gltf = Gltf::open("/Users/vladas/Downloads/Untitled.gltf")?;

        dbg!(gltf.scenes().len());

        for scene in gltf.scenes() {
            for node in scene.nodes() {
                println!("Node #{} has {} children", node.index(), node.children().count(),);
            }
        }

        let (document, buffers, images) = gltf::import("/Users/vladas/Downloads/Untitled.gltf")?;

        dbg!(document.meshes().len());

        assert_eq!(buffers.len(), document.buffers().count());
        assert_eq!(images.len(), document.images().count());

        Ok(())
    }
}
