use std::io::Write;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: Color) -> std::io::Result<()> {
    let r = (255.999 * pixel_color.x).min(255.0).max(0.0) as u8;
    let g = (255.999 * pixel_color.y).min(255.0).max(0.0) as u8;
    let b = (255.999 * pixel_color.z).min(255.0).max(0.0) as u8;

    writeln!(out, "{r} {g} {b}")?;
    Ok(())
}