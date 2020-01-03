use gl::types::*;

pub enum Format {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    LineStripAdj,
    LinesAdj,
    TriangleStrip,
    TriangleFan,
    Triangles,
    TriangleStripAdj,
    TriangleAdj,
    Patches,
    Quard
}

impl Format {
    pub fn value(&self) -> GLenum {
        match self {
            Format::Points => gl::POINTS,
            Format::LineStrip => gl::LINE_STRIP,
            Format::LineLoop => gl::LINE_LOOP,
            Format::Lines => gl::LINES,
            Format::LineStripAdj => gl::LINE_STRIP_ADJACENCY,
            Format::LinesAdj => gl::LINES_ADJACENCY,
            Format::TriangleStrip => gl::TRIANGLE_STRIP,
            Format::TriangleFan => gl::TRIANGLE_FAN,
            Format::Triangles => gl::TRIANGLES,
            Format::TriangleStripAdj => gl::TRIANGLE_STRIP_ADJACENCY,
            Format::TriangleAdj => gl::TRIANGLES_ADJACENCY,
            Format::Patches => gl::PATCHES,
            Format::Quard => gl::QUADS,
        }
    }
}