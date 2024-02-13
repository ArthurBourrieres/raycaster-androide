use macroquad::prelude::*;
use std::num::{self, *};

#[macroquad::main("raycaster-android")]
async fn main() -> anyhow::Result<()> {
    let world = vec![
        [
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ],
    ];

    let liste = vec![1, 22, 2, 3];

    let mut posX = 22.;
    let mut posY = 12.;

    let mut dirX = -1.;
    let mut dirY = 0.;

    let mut planeX = 0.;
    let mut planeY = 0.66;

    let mut time = 0;
    let mut old_time = 0;

    loop {
        clear_background(BLACK);

        for x in 1..screen_width() as i32 {
            let mut cameraX = 2. * x as f32 / screen_height() - 1.;

            let mut rayDirX = dirX + planeX * cameraX as f32;
            let mut rayDirY = dirY + planeY * cameraX as f32;

            let mut mapX = posX as i32;
            let mut mapY = posY as i32;

            let mut sideDistX: f32;
            let mut sideDistY: f32;

            let mut deltaDistX: f32 = 0.;
            let mut deltaDistY: f32 = 0.;

            if rayDirX != 0. {
                deltaDistX = (1. / rayDirX).abs();
            }
            if rayDirY != 0. {
                deltaDistY = (1. / rayDirY).abs();
            }

            let mut perpWallDist: f32;
            let mut stepX: i32;
            let mut stepY: i32;

            let mut hit = 0;
            let mut side = 0;

            if rayDirX < 0. {
                stepX = -1;
                sideDistX = (posX - mapX as f32) * deltaDistX;
            } else {
                stepX = 1;
                sideDistX = (mapX as f32 + 1.0 - posX) * deltaDistX;
            }
            if rayDirY < 0. {
                stepY = -1;
                sideDistY = (posY - mapY as f32) * deltaDistY;
            } else {
                stepY = 1;
                sideDistY = (mapY as f32 + 1.0 - posY) * deltaDistY;
            }

            while hit == 0 {
                if sideDistX < sideDistY {
                    sideDistX += deltaDistX;
                    mapX += stepX;
                    side = 0;
                } else {
                    sideDistY += deltaDistY;
                    mapY += stepY;
                    side = 1;
                }
                if &world[mapX as usize][mapY as usize] > &0 {
                    hit = 1;
                }
            }

            if side == 0 {
                perpWallDist = sideDistX - deltaDistX;
            } else {
                perpWallDist = sideDistY - deltaDistY;
            }

            let h = screen_height();
            let line_Height = h / perpWallDist;

            let mut drawStart = -line_Height / 2. + h / 2.;

            if drawStart < 0. {
                drawStart = 0.;
            }

            let mut drawEnd = line_Height / 2. + h / 2.;
            if drawEnd >= h {
                drawEnd = h - 1.;
            }

            let mut color = WHITE;

            let value_Color = world[mapX as usize][mapY as usize];

            match value_Color {
                0 => color = PINK,
                1 => color = RED,
                2 => color = GREEN,
                3 => color = BLUE,
                4 => color = YELLOW,
                5 => color = PINK,
                i32::MIN..=0_i32 | 5_i32..=i32::MAX => {
                    println!("HEY {}", value_Color);
                    todo!();
                }
            }

            draw_line(x as f32, drawStart, x as f32, drawEnd, 1 as f32, color);
        }

        let moveSpeed: f32 = 0.5;
        let rotSpeed: f32 = 0.05;

        if is_key_down(KeyCode::Up) {
            if world[(posX + dirX * moveSpeed) as usize][posY as usize] == 0 {
                posX += dirX * moveSpeed;
            }
            if world[posX as usize][(posY + dirY * moveSpeed) as usize] == 0 {
                posY += dirY * moveSpeed;
            }
        }
        if is_key_down(KeyCode::Down) {
            if world[(posX - dirX * moveSpeed) as usize][posY as usize] == 0 {
                posX -= dirX * moveSpeed;
            }
            if world[posX as usize][(posY - dirY * moveSpeed) as usize] == 0 {
                posY -= dirY * moveSpeed;
            }
        }

        if is_key_down(KeyCode::Right) {
            let rot = -rotSpeed;
            let oldDirX = dirX;
            dirX = dirX * rot.cos() - dirY * rot.sin();
            dirY = oldDirX * rot.sin() + dirY * rot.cos();
            let oldPlaneX = planeX;
            planeX = planeX * rot.cos() - planeY * rot.sin();
            planeY = oldPlaneX * rot.sin() + planeY * rot.cos();
        }

        if is_key_down(KeyCode::Left) {
            let oldDirX = dirX;
            dirX = dirX * rotSpeed.cos() - dirY * rotSpeed.sin();
            dirY = oldDirX * rotSpeed.sin() + dirY * rotSpeed.cos();
            let oldPlaneX = planeX;
            planeX = planeX * rotSpeed.cos() - planeY * rotSpeed.sin();
            planeY = oldPlaneX * rotSpeed.sin() + planeY * rotSpeed.cos();
        }

        fn bouton(x: f32, y: f32, w: f32, h: f32, color: Color) -> bool {
            draw_rectangle(x, y, w, h, color);
            let touchet = touches();
            for touch in touchet {
                if x < touch.position[0]
                    && touch.position[0] < x + w
                    && y < touch.position[1]
                    && touch.position[1] < y + h
                {
                    return true;
                }
            }

            return false;
        }

        if bouton(60., screen_height() - 120., 60., 60., WHITE) {
            let oldDirX = dirX;
            dirX = dirX * rotSpeed.cos() - dirY * rotSpeed.sin();
            dirY = oldDirX * rotSpeed.sin() + dirY * rotSpeed.cos();
            let oldPlaneX = planeX;
            planeX = planeX * rotSpeed.cos() - planeY * rotSpeed.sin();
            planeY = oldPlaneX * rotSpeed.sin() + planeY * rotSpeed.cos();
        }

        if bouton(180., screen_height() - 120., 60., 60., WHITE) {
            let oldDirX = dirX;
            dirX = dirX * rotSpeed.cos() - dirY * rotSpeed.sin();
            dirY = oldDirX * rotSpeed.sin() + dirY * rotSpeed.cos();
            let oldPlaneX = planeX;
            planeX = planeX * rotSpeed.cos() - planeY * rotSpeed.sin();
            planeY = oldPlaneX * rotSpeed.sin() + planeY * rotSpeed.cos();
        }

        if bouton(120., screen_height() - 60., 60., 60., WHITE) {
            if world[(posX - dirX * moveSpeed) as usize][posY as usize] == 0 {
                posX -= dirX * moveSpeed;
            }
            if world[posX as usize][(posY - dirY * moveSpeed) as usize] == 0 {
                posY -= dirY * moveSpeed;
            }
        }

        if bouton(120., screen_height() - 180., 60., 60., WHITE) {
            if world[(posX + dirX * moveSpeed) as usize][posY as usize] == 0 {
                posX += dirX * moveSpeed;
            }
            if world[posX as usize][(posY + dirY * moveSpeed) as usize] == 0 {
                posY += dirY * moveSpeed;
            }
        }

        next_frame().await
    }
}
