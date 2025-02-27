use crate::color::conv::IntoLinSrgba;
use crate::draw::drawing::DrawingContext;
use crate::draw::primitive::Primitive;
use crate::draw::properties::spatial::{self, dimension, orientation, position};
use crate::draw::properties::{
    ColorScalar, LinSrgba, SetColor, SetDimensions, SetOrientation, SetPosition,
};
use crate::draw::{self, theme, Drawing};
use crate::geom::{self, Vector2};
use crate::math::{BaseFloat, Zero};
use crate::text::{self, Align, Font, FontSize, Justify, Layout, Scalar, Wrap};

/// Properties related to drawing the **Text** primitive.
#[derive(Clone, Debug)]
pub struct Text<S = geom::scalar::Default> {
    spatial: spatial::Properties<S>,
    style: Style,
    // The byte range into the `Draw` context's text buffer.
    text: std::ops::Range<usize>,
}

/// Styling properties for the **Text** primitive.
#[derive(Clone, Debug, Default)]
pub struct Style {
    pub color: Option<LinSrgba>,
    pub glyph_colors: Vec<LinSrgba>, // Overrides `color` if non-empty.
    pub layout: text::layout::Builder,
}

/// The drawing context for the **Text** primitive.
pub type DrawingText<'a, S = geom::scalar::Default> = Drawing<'a, Text<S>, S>;

impl<S> Text<S> {
    /// Begin drawing some text.
    pub fn new(ctxt: DrawingContext<S>, text: &str) -> Self
    where
        S: Zero,
    {
        let start = ctxt.text_buffer.len();
        ctxt.text_buffer.push_str(text);
        let end = ctxt.text_buffer.len();
        let text = start..end;
        let spatial = Default::default();
        let style = Default::default();
        Text {
            spatial,
            style,
            text,
        }
    }

    // Apply the given function to the inner text layout.
    fn map_layout<F>(mut self, map: F) -> Self
    where
        F: FnOnce(text::layout::Builder) -> text::layout::Builder,
    {
        self.style.layout = map(self.style.layout);
        self
    }

    /// The font size to use for the text.
    pub fn font_size(self, size: FontSize) -> Self {
        self.map_layout(|l| l.font_size(size))
    }

    /// Specify whether or not text should be wrapped around some width and how to do so.
    ///
    /// The default value is `DEFAULT_LINE_WRAP`.
    pub fn line_wrap(self, line_wrap: Option<Wrap>) -> Self {
        self.map_layout(|l| l.line_wrap(line_wrap))
    }

    /// Specify that the **Text** should not wrap lines around the width.
    ///
    /// Shorthand for `builder.line_wrap(None)`.
    pub fn no_line_wrap(self) -> Self {
        self.map_layout(|l| l.no_line_wrap())
    }

    /// Line wrap the **Text** at the beginning of the first word that exceeds the width.
    ///
    /// Shorthand for `builder.line_wrap(Some(Wrap::Whitespace))`.
    pub fn wrap_by_word(self) -> Self {
        self.map_layout(|l| l.wrap_by_word())
    }

    /// Line wrap the **Text** at the beginning of the first character that exceeds the width.
    ///
    /// Shorthand for `builder.line_wrap(Some(Wrap::Character))`.
    pub fn wrap_by_character(self) -> Self {
        self.map_layout(|l| l.wrap_by_character())
    }

    /// A method for specifying the `Font` used for displaying the `Text`.
    pub fn font(self, font: Font) -> Self {
        self.map_layout(|l| l.font(font))
    }

    /// Describe the end along the *x* axis to which the text should be aligned.
    pub fn justify(self, justify: Justify) -> Self {
        self.map_layout(|l| l.justify(justify))
    }

    /// Align the text to the left of its bounding **Rect**'s *x* axis range.
    pub fn left_justify(self) -> Self {
        self.map_layout(|l| l.left_justify())
    }

    /// Align the text to the middle of its bounding **Rect**'s *x* axis range.
    pub fn center_justify(self) -> Self {
        self.map_layout(|l| l.center_justify())
    }

    /// Align the text to the right of its bounding **Rect**'s *x* axis range.
    pub fn right_justify(self) -> Self {
        self.map_layout(|l| l.right_justify())
    }

    /// Specify how much vertical space should separate each line of text.
    pub fn line_spacing(self, spacing: Scalar) -> Self {
        self.map_layout(|l| l.line_spacing(spacing))
    }

    /// Specify how the whole text should be aligned along the y axis of its bounding rectangle
    pub fn y_align(self, align: Align) -> Self {
        self.map_layout(|l| l.y_align(align))
    }

    /// Align the top edge of the text with the top edge of its bounding rectangle.
    pub fn align_top(self) -> Self {
        self.map_layout(|l| l.align_top())
    }

    /// Align the middle of the text with the middle of the bounding rect along the y axis.
    ///
    /// This is the default behaviour.
    pub fn align_middle_y(self) -> Self {
        self.map_layout(|l| l.align_middle_y())
    }

    /// Align the bottom edge of the text with the bottom edge of its bounding rectangle.
    pub fn align_bottom(self) -> Self {
        self.map_layout(|l| l.align_bottom())
    }

    /// Set all the parameters via an existing `Layout`
    pub fn layout(self, layout: &Layout) -> Self {
        self.map_layout(|l| l.layout(layout))
    }

    /// Specify the entire styling for the **Text**.
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set a color for each glyph.
    /// Colors unspecified glyphs using the drawing color.
    pub fn glyph_colors(mut self, colors: Vec<LinSrgba>) -> Self {
        self.style.glyph_colors = colors;
        self
    }
}

impl<'a, S> DrawingText<'a, S>
where
    S: BaseFloat,
{
    /// The font size to use for the text.
    pub fn font_size(self, size: text::FontSize) -> Self {
        self.map_ty(|ty| ty.font_size(size))
    }

    /// Specify that the **Text** should not wrap lines around the width.
    pub fn no_line_wrap(self) -> Self {
        self.map_ty(|ty| ty.no_line_wrap())
    }

    /// Line wrap the **Text** at the beginning of the first word that exceeds the width.
    pub fn wrap_by_word(self) -> Self {
        self.map_ty(|ty| ty.wrap_by_word())
    }

    /// Line wrap the **Text** at the beginning of the first character that exceeds the width.
    pub fn wrap_by_character(self) -> Self {
        self.map_ty(|ty| ty.wrap_by_character())
    }

    /// A method for specifying the `Font` used for displaying the `Text`.
    pub fn font(self, font: text::Font) -> Self {
        self.map_ty(|ty| ty.font(font))
    }

    /// Build the **Text** with the given **Style**.
    pub fn with_style(self, style: Style) -> Self {
        self.map_ty(|ty| ty.with_style(style))
    }

    /// Describe the end along the *x* axis to which the text should be aligned.
    pub fn justify(self, justify: text::Justify) -> Self {
        self.map_ty(|ty| ty.justify(justify))
    }

    /// Align the text to the left of its bounding **Rect**'s *x* axis range.
    pub fn left_justify(self) -> Self {
        self.map_ty(|ty| ty.left_justify())
    }

    /// Align the text to the middle of its bounding **Rect**'s *x* axis range.
    pub fn center_justify(self) -> Self {
        self.map_ty(|ty| ty.center_justify())
    }

    /// Align the text to the right of its bounding **Rect**'s *x* axis range.
    pub fn right_justify(self) -> Self {
        self.map_ty(|ty| ty.right_justify())
    }

    /// Specify how much vertical space should separate each line of text.
    pub fn line_spacing(self, spacing: text::Scalar) -> Self {
        self.map_ty(|ty| ty.line_spacing(spacing))
    }

    /// Specify how the whole text should be aligned along the y axis of its bounding rectangle
    pub fn y_align_text(self, align: Align) -> Self {
        self.map_ty(|ty| ty.y_align(align))
    }

    /// Align the top edge of the text with the top edge of its bounding rectangle.
    pub fn align_text_top(self) -> Self {
        self.map_ty(|ty| ty.align_top())
    }

    /// Align the middle of the text with the middle of the bounding rect along the y axis.
    ///
    /// This is the default behaviour.
    pub fn align_text_middle_y(self) -> Self {
        self.map_ty(|ty| ty.align_middle_y())
    }

    /// Align the bottom edge of the text with the bottom edge of its bounding rectangle.
    pub fn align_text_bottom(self) -> Self {
        self.map_ty(|ty| ty.align_bottom())
    }

    /// Set all the parameters via an existing `Layout`
    pub fn layout(self, layout: &Layout) -> Self {
        self.map_ty(|ty| ty.layout(layout))
    }

    /// Set a color for each glyph.
    /// Colors unspecified glyphs using the drawing color.
    pub fn glyph_colors<I, C>(self, glyph_colors: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: IntoLinSrgba<ColorScalar>,
    {
        let glyph_colors = glyph_colors
            .into_iter()
            .map(|c| c.into_lin_srgba())
            .collect();

        self.map_ty(|ty| ty.glyph_colors(glyph_colors))
    }
}

impl draw::renderer::RenderPrimitive for Text<f32> {
    fn render_primitive(
        self,
        ctxt: draw::renderer::RenderContext,
        mesh: &mut draw::Mesh,
    ) -> draw::renderer::PrimitiveRender {
        let Text {
            spatial,
            style,
            text,
        } = self;
        let Style {
            color,
            glyph_colors,
            layout,
        } = style;
        let layout = layout.build();
        let (maybe_x, maybe_y, maybe_z) = (
            spatial.dimensions.x,
            spatial.dimensions.y,
            spatial.dimensions.z,
        );
        assert!(
            maybe_z.is_none(),
            "z dimension support for text is unimplemented"
        );
        let w = maybe_x
            .map(|s| <f32 as crate::math::NumCast>::from(s).unwrap())
            .unwrap_or(200.0);
        let h = maybe_y
            .map(|s| <f32 as crate::math::NumCast>::from(s).unwrap())
            .unwrap_or(200.0);
        let rect: geom::Rect = geom::Rect::from_wh(Vector2 { x: w, y: h });
        let color = color.unwrap_or_else(|| ctxt.theme.fill_lin_srgba(&theme::Primitive::Text));

        let text_str = &ctxt.text_buffer[text.clone()];
        let text = text::text(text_str).layout(&layout).build(rect);

        // Queue the glyphs to be cached
        let font_id = text::font::id(text.font());
        let positioned_glyphs: Vec<_> = text
            .rt_glyphs(
                ctxt.output_attachment_size,
                ctxt.output_attachment_scale_factor,
            )
            .collect();
        for glyph in positioned_glyphs.iter() {
            ctxt.glyph_cache.queue_glyph(font_id.index(), glyph.clone());
        }

        // Cache the enqueued glyphs within the pixel buffer.
        let (glyph_cache_w, _) = ctxt.glyph_cache.dimensions();
        {
            let draw::renderer::RenderContext {
                glyph_cache:
                    &mut draw::renderer::GlyphCache {
                        ref mut cache,
                        ref mut pixel_buffer,
                        ref mut requires_upload,
                        ..
                    },
                ..
            } = ctxt;
            let glyph_cache_w = glyph_cache_w as usize;
            let res = cache.cache_queued(|rect, data| {
                let width = (rect.max.x - rect.min.x) as usize;
                let height = (rect.max.y - rect.min.y) as usize;
                let mut dst_ix = rect.min.y as usize * glyph_cache_w + rect.min.x as usize;
                let mut src_ix = 0;
                for _ in 0..height {
                    let dst_range = dst_ix..dst_ix + width;
                    let src_range = src_ix..src_ix + width;
                    let dst_slice = &mut pixel_buffer[dst_range];
                    let src_slice = &data[src_range];
                    dst_slice.copy_from_slice(src_slice);
                    dst_ix += glyph_cache_w;
                    src_ix += width;
                }
                *requires_upload = true;
            });
            if let Err(err) = res {
                eprintln!("failed to cache queued glyphs: {}", err);
            }
        }

        // Determine the transform to apply to all points.
        let global_transform = ctxt.transform;
        let local_transform = spatial.position.transform() * spatial.orientation.transform();
        let transform = global_transform * local_transform;

        // A function for converting RustType rects to nannou rects.
        let scale_factor = ctxt.output_attachment_scale_factor;
        let (out_w, out_h) = ctxt.output_attachment_size.into();
        let [half_out_w, half_out_h] = [out_w as f32 / 2.0, out_h as f32 / 2.0];
        let to_nannou_rect = |screen_rect: text::rt::Rect<i32>| {
            let l = screen_rect.min.x as f32 / scale_factor - half_out_w;
            let r = screen_rect.max.x as f32 / scale_factor - half_out_w;
            let t = -(screen_rect.min.y as f32 / scale_factor - half_out_h);
            let b = -(screen_rect.max.y as f32 / scale_factor - half_out_h);
            geom::Rect::from_corners(geom::pt2(l, b), geom::pt2(r, t))
        };

        // Extend the mesh with a rect for each displayed glyph.
        // If `glyph_colors` contains insufficient colors, use `color` for the remaining glyphs.
        for (g, g_color) in positioned_glyphs
            .iter()
            .zip(glyph_colors.iter().chain(std::iter::repeat(&color)))
        {
            if let Ok(Some((uv_rect, screen_rect))) = ctxt.glyph_cache.rect_for(font_id.index(), &g)
            {
                let rect = to_nannou_rect(screen_rect);

                // Create a mesh-compatible vertex from the position and tex_coords.
                let v = |position, tex_coords: [f32; 2]| -> draw::mesh::Vertex {
                    let p = geom::Point3::from(position);
                    let p = cgmath::Transform::transform_point(&transform, p.into());
                    let point = draw::mesh::vertex::Point::from(p);
                    draw::mesh::vertex::new(point, g_color.to_owned(), tex_coords.into())
                };

                // The sides of the UV rect.
                let uv_l = uv_rect.min.x;
                let uv_t = uv_rect.min.y;
                let uv_r = uv_rect.max.x;
                let uv_b = uv_rect.max.y;

                // Insert the vertices.
                let bottom_left = v(rect.bottom_left(), [uv_l, uv_b]);
                let bottom_right = v(rect.bottom_right(), [uv_r, uv_b]);
                let top_left = v(rect.top_left(), [uv_l, uv_t]);
                let top_right = v(rect.top_right(), [uv_r, uv_t]);
                let start_ix = mesh.points().len() as u32;
                mesh.push_vertex(top_left);
                mesh.push_vertex(bottom_left);
                mesh.push_vertex(bottom_right);
                mesh.push_vertex(top_right);

                // Now the indices.
                let tl_ix = start_ix;
                let bl_ix = start_ix + 1;
                let br_ix = start_ix + 2;
                let tr_ix = start_ix + 3;
                mesh.push_index(tl_ix);
                mesh.push_index(bl_ix);
                mesh.push_index(br_ix);
                mesh.push_index(tl_ix);
                mesh.push_index(br_ix);
                mesh.push_index(tr_ix);
            }
        }

        draw::renderer::PrimitiveRender::text()
    }
}

impl<S> SetOrientation<S> for Text<S> {
    fn properties(&mut self) -> &mut orientation::Properties<S> {
        SetOrientation::properties(&mut self.spatial)
    }
}

impl<S> SetPosition<S> for Text<S> {
    fn properties(&mut self) -> &mut position::Properties<S> {
        SetPosition::properties(&mut self.spatial)
    }
}

impl<S> SetDimensions<S> for Text<S> {
    fn properties(&mut self) -> &mut dimension::Properties<S> {
        SetDimensions::properties(&mut self.spatial)
    }
}

impl<S> SetColor<ColorScalar> for Text<S> {
    fn rgba_mut(&mut self) -> &mut Option<LinSrgba> {
        SetColor::rgba_mut(&mut self.style.color)
    }
}

// Primitive conversions.

impl<S> From<Text<S>> for Primitive<S> {
    fn from(prim: Text<S>) -> Self {
        Primitive::Text(prim)
    }
}

impl<S> Into<Option<Text<S>>> for Primitive<S> {
    fn into(self) -> Option<Text<S>> {
        match self {
            Primitive::Text(prim) => Some(prim),
            _ => None,
        }
    }
}
