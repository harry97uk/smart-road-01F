use sdl2::{ render::{ WindowCanvas }, pixels::Color, rect::Rect };

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
    intersection: &Intersection
) -> Result<(), String> {
    // Draw lanes
    for (index, lane) in intersection.lanes.iter().enumerate() {
        // Draw lane
        canvas.set_draw_color(Color::WHITE);
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

        canvas.draw_rect(lane_rect).unwrap();
        canvas.draw_rect(opposite_lane_rect).unwrap();
    }
    Ok(())
}

fn render_cars(canvas: &mut WindowCanvas, intersection: &Intersection) -> Result<(), String> {
    canvas.set_draw_color(Color::RED);
    for car in &intersection.vehicles {
        let car_rect = Rect::new(car.position.x, car.position.y, VEHICLE_WIDTH, VEHICLE_HEIGHT);
        canvas.draw_rect(car_rect).unwrap();
    }
    Ok(())
}

pub fn render(canvas: &mut WindowCanvas, intersection: &Intersection) -> Result<(), String> {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    render_intersection(canvas, intersection)?;
    render_cars(canvas, intersection)?;
    canvas.present();
    Ok(())
}
