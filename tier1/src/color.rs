use std::ops::Index;

#[derive(Clone, PartialEq, Eq, Copy, Debug)]
pub struct Color {
	red: u8,
	green: u8,
	blue: u8,
	alpha: u8
}

impl From<u32> for Color {
	fn from( value: u32 ) -> Self {
		Color {
			red: ( ( value >> 24 ) & 0xFF ) as u8,
			green: ( ( value >> 16 ) & 0xFF ) as u8,
			blue: ( ( value >> 8 ) & 0xFF ) as u8,
			alpha: ( ( value >> 0 ) & 0xFF ) as u8
		}
	}
}

impl From<Color> for u32 {
	fn from(value: Color) -> Self {
		( ( value.red as u32   ) << 24 ) |
		( ( value.green as u32 ) << 16 ) |
		( ( value.blue as u32  ) << 8  ) |
		( ( value.alpha as u32 ) << 0  )
	}
}

impl From<Vec<u8>> for Color {
	fn from( value: Vec<u8> ) -> Self {
		Color {
			red: value[0],
			green: value[1],
			blue: value[2],
			alpha: value[3]
		}
	}
}

impl Index<u8> for Color {
	type Output = u8;

	fn index(&self, index: u8) -> &Self::Output {
		match index {
			0 => &self.red,
			1 => &self.green,
			2 => &self.blue,
			3 => &self.alpha,
			num => panic!("got asked for component {num}!")
		}
	}
}

#[test]
fn color_test() {
	let color = Color { red: 0xFF, green: 0xAB, blue: 0xCD, alpha: 0xEF };
	let colorNum: u32 = u32::from(color);

	assert_eq!( 0xFFABCDEF, colorNum, "Checking if `Color -> u32` works" );
	assert_eq!( color, Color::from(colorNum), "Checking if `u32 -> Color` works" );
}
