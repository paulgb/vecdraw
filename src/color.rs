use palette::{Srgb, Srgba};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Color(pub u32);

impl Into<Color> for Srgb<u8> {
    fn into(self) -> Color {
        Color(*bytemuck::from_bytes(&[
            self.red, self.green, self.blue, 0xff,
        ]))
    }
}

impl Into<Color> for Srgba<u8> {
    fn into(self) -> Color {
        Color(*bytemuck::from_bytes(&[
            self.red, self.green, self.blue, self.alpha,
        ]))
    }
}
