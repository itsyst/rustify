use macroquad::prelude::*;

#[macroquad::main("Rustify")]

async fn main() {
    const SCR_W: f32 = 1000.0;
    const SCR_H: f32 = 620.5;
    let mut dx = 25.;
    let mut dy = 7.;

    let mut head_x = SCR_W / 2.;
    let mut head_y = SCR_H / 2.;

    let bytes = include_bytes!("../resources/head.png");

    let head_texture = Texture2D::from_file_with_format(bytes, None);

    let head_ratio = head_texture.width() / head_texture.height();
    let head_width = 100.;
    let head_height = 100. / head_ratio;

    let max_x = SCR_W - head_width / 2.;
    let min_x = head_width / 2.;
    let max_y = SCR_H - head_height * 1.5;
    let min_y = head_height;

    let background =
        Texture2D::from_file_with_format(include_bytes!("../resources/background.png"), None);

    // build camera with following coordinate system:
    // (0., 0)     .... (SCR_W, 0.)
    // (0., SCR_H) .... (SCR_W, SCR_H)
    set_camera(&Camera2D {
        zoom: vec2(1. / SCR_W * 2., -1. / SCR_H * 2.),
        target: vec2(SCR_W / 2., SCR_H / 2.),
        ..Default::default()
    });

    loop {
        clear_background(WHITE);

        let delta = get_frame_time();

        // Move head
        head_x += dx * delta;
        head_y += dy * delta;

        // Change X direction
        if head_x <= min_x || head_x > (max_x - head_width) {
            dx *= -1.;
        }
        // Change Y direction
        if head_y <= min_y || head_y > (max_y - head_height) {
            dy *= -1.;
        }

        draw_texture_ex(
            background,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(SCR_W, SCR_H)),
                ..Default::default()
            },
        );

        draw_texture_ex(
            head_texture,
            head_x / 20.0,
            head_y / 20.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(head_width, head_height)),
                flip_x: dx > 0.,
                ..Default::default()
            },
        );

        next_frame().await
    }
}
