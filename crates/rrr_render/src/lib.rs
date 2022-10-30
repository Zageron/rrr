use anyhow::Result;
use pixels::{
    raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle},
    wgpu::{Color, RequestAdapterOptions},
    Pixels, PixelsBuilder, SurfaceTexture,
};
use rrr_chart::RuntimeNote;
use rrr_noteskin::Noteskin;
use rrr_types::ReceptorPosition;

pub mod field;

#[derive(Debug)]
pub struct Renderer {
    pub pixels: Pixels,
    pub width: u32,
    pub height: u32,
}

impl Renderer {
    pub fn render_field<'a>(
        &mut self,
        view: impl IntoIterator<Item = (&'a u32, &'a RuntimeNote)>,
        chart_progress: u32,
        start_position: i32,
        end_position: i32,
        receptor_position: ReceptorPosition,
        time_on_screen: u32,
        noteskin: &Noteskin,
        gap: u8,
        ms_offset: i32,
    ) -> Result<()> {
        let frame: &mut [u8] = self.pixels.get_frame_mut();
        field::clear(frame);

        let offset = self.width as f32 / 2.0 - noteskin.note_width as f32 * 0.5;

        field::draw_receptors(
            noteskin,
            frame,
            offset,
            receptor_position
                .0
                .saturating_sub(noteskin.note_height.saturating_div(2) as u32) as f32,
            gap,
            self.width,
            self.height,
        );

        field::draw_notes(
            view,
            time_on_screen,
            chart_progress,
            ms_offset,
            offset,
            frame,
            noteskin,
            gap,
            self.width,
            self.height,
            start_position,
            end_position,
        );

        self.pixels.render()?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct RendererBuilder<'win, W: HasRawWindowHandle + HasRawDisplayHandle> {
    color: Color,
    width: u32,
    height: u32,
    window: &'win W,
}

impl<'win, W: HasRawWindowHandle + HasRawDisplayHandle> RendererBuilder<'win, W> {
    pub fn new(width: u32, height: u32, window: &'win W) -> Self {
        let default_clear_color = pixels::wgpu::Color {
            r: 0.,
            g: 0.,
            b: 0.,
            a: 1.0,
        };

        Self {
            width,
            height,
            window,
            color: default_clear_color,
        }
    }

    pub fn with_clear_color(&mut self, color: Color) {
        self.color = color;
    }

    pub async fn build(self) -> Result<Renderer> {
        let surface_texture = SurfaceTexture::new(self.width, self.height, self.window);
        let pixels = PixelsBuilder::new(self.width, self.height, surface_texture)
            .clear_color(self.color)
            .request_adapter_options(RequestAdapterOptions {
                power_preference: pixels::wgpu::PowerPreference::HighPerformance,
                ..RequestAdapterOptions::default()
            })
            .build_async()
            .await?;

        Ok(Renderer {
            pixels,
            width: self.width,
            height: self.height,
        })
    }
}
