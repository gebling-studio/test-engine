use ui_proc::view;

use crate::{
    gm::{
        axis::Axis,
        color::Color,
        flat::{Point, Size},
    },
    render::data::PathData,
    ui::{ViewCallbacks, view::ViewFrame},
    window::Window,
};

/// Path rendering is currently disconnected. `UIDrawer` does not draw paths,
/// so this view and everything built on it produces no visuals.
/// Planned to be reimplemented later.
#[view]
pub struct DrawingView {
    prev_size:   Size,
    pub rescale: bool,
    paths:       Vec<PathData>,
}

impl ViewCallbacks for DrawingView {
    fn update(&mut self) {
        if self.prev_size == self.size() {
            return;
        }

        self.prev_size = self.size();

        self.update_buffers();
    }
}

impl DrawingView {
    pub fn paths(&self) -> &[PathData] {
        &self.paths
    }

    pub fn add_path<Container, P>(&mut self, path: Container, color: Color) -> &mut Self
    where
        P: Into<Point>,
        Container: IntoIterator<Item = P>, {
        let points = path.into_iter().map(Into::into).collect();

        let path = self.process_points(points);
        if path.is_empty() {
            return self;
        }

        self.paths.push(PathData::new(
            color,
            Window::render_size(),
            self.absolute_frame().origin,
            self.z_position(),
            &path,
        ));

        self.update_buffers();

        self
    }

    fn process_points(&self, path: Vec<Point>) -> Vec<Point> {
        if !self.rescale {
            return path;
        }

        let max_x = path.iter().map(|p| p.x).fold(f32::NAN, f32::max);
        let max_y = path.iter().map(|p| p.y).fold(f32::NAN, f32::max);

        let path_size = Size::new(max_x, max_y);

        let fitted_size = path_size.fit_in_rect::<{ Axis::X }>(self.frame()).size;

        let ratios = path_size.ratios(fitted_size);

        path.into_iter().map(|point| point * ratios).collect()
    }

    fn update_buffers(&mut self) {
        let pos = self.absolute_frame().origin;
        for path in &mut self.paths {
            path.resize(pos);
        }
    }

    pub fn remove_all_paths(&mut self) {
        self.paths.clear();
        self.update_buffers();
    }
}
