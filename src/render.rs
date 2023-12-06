use sdl2::{ render::{ WindowCanvas }, pixels::Color, rect::Rect };

use crate::{ intersection::{ Intersection, Direction }, WINDOW_HEIGHT, WINDOW_WIDTH };

const VERTICAL_LANE_WIDTH: u32 = WINDOW_WIDTH / 18;
const VERTICAL_LANE_HEIGHT: u32 = WINDOW_HEIGHT / 3;
const HORIZONTAL_LANE_HEIGHT: u32 = WINDOW_HEIGHT / 18;
const HORIZONTAL_LANE_WIDTH: u32 = WINDOW_WIDTH / 3;

fn render_intersection(canvas: &mut WindowCanvas, intersection: &Intersection) {
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
}

pub fn render(canvas: &mut WindowCanvas, intersection: &Intersection) -> Result<(), String> {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    render_intersection(canvas, intersection);
    canvas.present();
    Ok(())
}
