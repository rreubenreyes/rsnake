use piston_window::{rectangle, Context, G2d};
use piston_window::types::{Color};

const BLOCK_SIZE: f64 = 25.0;

fn scale(num: i32) -> f64 { (num as f64) * BLOCK_SIZE }
fn rect(color: Color, ctx: &Context, g2d: &mut G2d) -> impl Fn(f64, f64, f64, f64) {
		|x, y, lx, ly| rectangle(color, [x, y, lx, ly], ctx, g2d)
}
pub fn to_coord(game_coord: (i32, i32)) -> (f64, f64) {
		let (x, y) = game_coord;
		((x as f64) * BLOCK_SIZE, (y as f64) * BLOCK_SIZE)
}

pub fn draw_block(color: Color, point: (i32, i32), ctx: &Context, g2d: &mut G2d) {
	let (x, y) = to_coord(point);
	let bounds = [x, y, BLOCK_SIZE, BLOCK_SIZE];

	rectangle(color, bounds, ctx.transform, g2d);
}
