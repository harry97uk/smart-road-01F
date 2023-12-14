use sdl2::{ render::{ WindowCanvas, Texture }, pixels::Color, rect::{ Rect, Point } };

use crate::{
    intersection::{ Intersection, Direction },
    WINDOW_HEIGHT,
    WINDOW_WIDTH,
    vehicle::{ VEHICLE_WIDTH, VEHICLE_HEIGHT },
};

pub const VERTICAL_LANE_WIDTH: u32 = WINDOW_WIDTH / 18;
const VERTICAL_LANE_HEIGHT: u32 = WINDOW_HEIGHT / 3;
pub const HORIZONTAL_LANE_HEIGHT: u32 = WINDOW_HEIGHT / 18;
const HORIZONTAL_LANE_WIDTH: u32 = WINDOW_WIDTH / 3;

fn render_intersection(
    canvas: &mut WindowCanvas,
    intersection: &Intersection,
    road_texture: &Texture
) -> Result<(), String> {
    // Draw lanes
    for (index, lane) in intersection.lanes.iter().enumerate() {
        // Draw lane
        canvas.set_draw_color(Color::WHITE);
        let lane_src = Rect::new(
            0,
            0,
            road_texture.query().width / 6,
            road_texture.query().height / 2
        );
        let mut angle = 0.0;
        let mut lane_rect = Rect::new(0, 0, 0, 0);
        let mut opposite_lane_rect = Rect::new(0, 0, 0, 0);
        let m = index % 3;
        match lane.origin {
            Direction::North => {
                lane_rect.set_x(
                    (WINDOW_WIDTH as i32) / 3 + ((VERTICAL_LANE_WIDTH * (m as u32)) as i32)
                );
                lane_rect.set_height(VERTICAL_LANE_HEIGHT);
                lane_rect.set_width(VERTICAL_LANE_WIDTH);
                opposite_lane_rect.set_x(
                    (WINDOW_WIDTH as i32) / 3 + ((VERTICAL_LANE_WIDTH * (m as u32)) as i32)
                );
                opposite_lane_rect.set_y(((WINDOW_HEIGHT as i32) * 2) / 3);
                opposite_lane_rect.set_height(VERTICAL_LANE_HEIGHT);
                opposite_lane_rect.set_width(VERTICAL_LANE_WIDTH);
            }
            Direction::South => {
                lane_rect.set_y(((WINDOW_HEIGHT as i32) * 2) / 3);
                lane_rect.set_x(
                    ((WINDOW_WIDTH as i32) * 2) / 3 -
                        (VERTICAL_LANE_WIDTH as i32) -
                        ((VERTICAL_LANE_WIDTH * (m as u32)) as i32)
                );
                lane_rect.set_height(VERTICAL_LANE_HEIGHT);
                lane_rect.set_width(VERTICAL_LANE_WIDTH);
                opposite_lane_rect.set_x(
                    ((WINDOW_WIDTH as i32) * 2) / 3 -
                        (VERTICAL_LANE_WIDTH as i32) -
                        ((VERTICAL_LANE_WIDTH * (m as u32)) as i32)
                );
                opposite_lane_rect.set_height(VERTICAL_LANE_HEIGHT);
                opposite_lane_rect.set_width(VERTICAL_LANE_WIDTH);
                angle = 180.0;
            }
            Direction::East => {
                lane_rect.set_x(((WINDOW_WIDTH as i32) * 2) / 3);
                lane_rect.set_y(
                    (WINDOW_HEIGHT as i32) / 3 + ((HORIZONTAL_LANE_HEIGHT * (m as u32)) as i32)
                );
                lane_rect.set_height(HORIZONTAL_LANE_HEIGHT);
                lane_rect.set_width(HORIZONTAL_LANE_WIDTH);
                opposite_lane_rect.set_y(
                    (WINDOW_HEIGHT as i32) / 3 + ((HORIZONTAL_LANE_HEIGHT * (m as u32)) as i32)
                );
                opposite_lane_rect.set_height(HORIZONTAL_LANE_HEIGHT);
                opposite_lane_rect.set_width(HORIZONTAL_LANE_WIDTH);
                angle = 180.0;
            }
            Direction::West => {
                lane_rect.set_y(
                    ((WINDOW_HEIGHT as i32) * 2) / 3 -
                        (HORIZONTAL_LANE_HEIGHT as i32) -
                        ((HORIZONTAL_LANE_HEIGHT * (m as u32)) as i32)
                );
                lane_rect.set_height(HORIZONTAL_LANE_HEIGHT);
                lane_rect.set_width(HORIZONTAL_LANE_WIDTH);
                opposite_lane_rect.set_x(((WINDOW_WIDTH as i32) * 2) / 3);
                opposite_lane_rect.set_y(
                    ((WINDOW_HEIGHT as i32) * 2) / 3 -
                        (HORIZONTAL_LANE_HEIGHT as i32) -
                        ((HORIZONTAL_LANE_HEIGHT * (m as u32)) as i32)
                );
                opposite_lane_rect.set_height(HORIZONTAL_LANE_HEIGHT);
                opposite_lane_rect.set_width(HORIZONTAL_LANE_WIDTH);
            }
        }

        canvas.copy_ex(road_texture, lane_src, lane_rect, angle, None, false, false).unwrap();
        canvas
            .copy_ex(road_texture, lane_src, opposite_lane_rect, angle, None, false, false)
            .unwrap();
        canvas.draw_rect(lane_rect).unwrap();
        canvas.draw_rect(opposite_lane_rect).unwrap();
    }
    let centre_square = Rect::new(
        (WINDOW_WIDTH / 3) as i32,
        (WINDOW_HEIGHT / 3) as i32,
        WINDOW_WIDTH / 3,
        WINDOW_HEIGHT / 3
    );
    let other_src = Rect::new(
        0,
        0,
        road_texture.query().width / 7,
        road_texture.query().height / 3
    );
    canvas.copy(road_texture, other_src, centre_square).unwrap();
    Ok(())
}

fn render_cars(
    canvas: &mut WindowCanvas,
    intersection: &Intersection,
    car_texture: &Texture
) -> Result<(), String> {
    canvas.set_draw_color(Color::RED);
    for car in &intersection.vehicles {
        if car.colliding {
            canvas.set_draw_color(Color::GREEN);
        } else {
            canvas.set_draw_color(Color::RED);
        }
        let mut angle = 0.0;
        let actual_rect = Rect::new(car.position.x, car.position.y, car.width, car.height);
        let mut screen_rect = Rect::new(
            car.position.x,
            car.position.y,
            VEHICLE_WIDTH,
            VEHICLE_HEIGHT
        );
        let src = Rect::new(
            0,
            ((car_texture.query().height * 3) / 5) as i32,
            car_texture.query().width,
            (car_texture.query().height * 2) / 5
        );
        match car.facing {
            Direction::North => {
                screen_rect.set_x(
                    car.position.x - (VEHICLE_WIDTH.abs_diff(VEHICLE_HEIGHT) as i32) / 2
                );
                screen_rect.set_y(car.position.y + 5);
                angle = 90.0;
            }
            Direction::South => {
                screen_rect.set_x(
                    car.position.x - (VEHICLE_WIDTH.abs_diff(VEHICLE_HEIGHT) as i32) / 2
                );
                screen_rect.set_y(car.position.y + 5);
                angle = 270.0;
            }
            Direction::East => {
                angle = 180.0;
            }
            Direction::West => {}
        }
        let perim = Rect::from_center(
            screen_rect.center(),
            VEHICLE_HEIGHT.max(VEHICLE_WIDTH) + 5,
            VEHICLE_HEIGHT.max(VEHICLE_WIDTH) + 5
        );
        canvas.copy_ex(car_texture, src, screen_rect, angle, None, false, false)?;
        canvas.draw_rect(actual_rect).unwrap();
        canvas.draw_rect(perim).unwrap();
    }
    Ok(())
}

pub fn render(
    canvas: &mut WindowCanvas,
    intersection: &Intersection,
    car_texture: &Texture,
    road_texture: &Texture
) -> Result<(), String> {
    canvas.set_draw_color(Color { r: 0, g: 100, b: 0, a: 1 });
    canvas.clear();
    render_intersection(canvas, intersection, road_texture)?;
    render_cars(canvas, intersection, car_texture)?;
    canvas.present();
    Ok(())
}
