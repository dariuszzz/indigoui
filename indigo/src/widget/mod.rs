pub mod text;


pub use text::*;

pub mod vertical_con;
pub use vertical_con::*;

pub mod image;
pub use crate::widget::image::*;

use crate::{
    app::App,
    context::{IndigoContext, RenderContext},
    error::IndigoError,
    event::{WidgetEvent},
    graphics::IndigoRenderer, view::View,

};

pub struct Layout {
    pub origin: (f32, f32, f32),
    pub available_space: (f32, f32),
}

pub trait Widget<A, V, R>: std::any::Any
where
    A: App<R>,
    V: View<A, R>,
    R: IndigoRenderer,
{
    fn handle_event(
        &mut self,
        _ctx: &mut IndigoContext<'_, '_, A, V, R>,
        _view: &mut V,
        _event: WidgetEvent,
    ) -> Result<(), IndigoError<R::ErrorMessage>>;

    fn generate_mesh(
        &self,
        _ctx: &mut RenderContext<'_, '_, A, V, R>,
        _view: &mut V,
        _layout: Layout,
    ) -> Result<Vec<R::RenderCommand>, IndigoError<R::ErrorMessage>> {
        Ok(Vec::new())
    }
}