use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

use crate::core::components::sprite::{SpriteRect, SpriteComponent};
use crate::core::components::transform::TransformComponent;

//converts sprite rect to sdl rect
pub fn spriterect_to_sdlrect(srect : &SpriteRect) -> Rect {

    Rect::new(srect.x, srect.y, srect.width, srect.height)
}

pub fn RenderSprite(transform : &TransformComponent, sprite : &SpriteComponent, canvas : &mut Canvas<Window>) {

    let src_rect = spriterect_to_sdlrect(&sprite.src_rect);

    //making origin at center of sprite
    let x = transform.position.x - transform.size.x / 2.0;
    let y = transform.position.y - transform.size.y / 2.0;

    let dest_rect = Rect::new(x as i32, y as i32, transform.size.x as u32 , transform.size.y as u32);

    canvas.copy_ex(&sprite.texture, Some(src_rect), Some(dest_rect),
                        transform.angle.into(), None, sprite.flip_horizontal, sprite.flip_vertical).unwrap();

}
