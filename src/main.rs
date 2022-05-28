use std::f32::consts::PI;
use plotters::prelude::*;
use rand::prelude::*;
use rand_distr::Normal;
use rand_distr::num_traits::ToPrimitive;

const MIN: i32 = -35;
const MAX: i32 = 35;

fn get_sample_points(n: usize, mean: (f32, f32)) -> Vec<(i32, i32)> {
    let mut points = Vec::with_capacity(n);
    let dist_x = Normal::new(mean.0, 5.0f32).unwrap();
    let dist_y = Normal::new(mean.1, 5.0f32).unwrap();

    for _ in 0..n {
        let x = thread_rng().sample(&dist_x).to_i32().unwrap();
        let y = thread_rng().sample(&dist_y).to_i32().unwrap();
        points.push((x.clamp(MIN, MAX), y.clamp(MIN, MAX)))
    }
    points
}

fn is_point_within_dist(src: (i32, i32), dst: (i32, i32), dist: f32) -> bool {
    let dx = src.0 - dst.0;
    let dy = src.1 - dst.1;
    ((dx * dx + dy * dy) as f32) < dist * dist
}

fn find_points_within_distance(src: (i32, i32), dist: f32, points: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    for &(x, y) in points {
        let dx = (x - src.0) as f32;
        let dy = (y - src.1) as f32;
        if dx * dx + dy * dy <= dist * dist {
            result.push((x, y));
        }
    }
    result
}

fn angle_from_to(a: (i32, i32), b: (i32, i32)) -> f32 {
    let dx = (b.0 - a.0) as f32;
    let dy = (b.1 - a.1) as f32;
    let angle = dy.atan2(dx);
    if angle < 0.0 {
        angle + 2.0 * PI
    } else {
        angle
    }
}

fn angle_between_three_points(old: (i32, i32), curr: (i32, i32), new: (i32, i32)) -> f32 {
    let dir_to_old = (old.0 - curr.0, old.1 - curr.1);
    let dir_to_new = (new.0 - curr.0, new.1 - curr.1);

    let angle = ((dir_to_old.0 * dir_to_new.1 - dir_to_old.1 * dir_to_new.0) as f32).atan2((dir_to_old.0 * dir_to_new.0 + dir_to_old.1 * dir_to_new.1) as f32);
    let angle = 2.0 * PI - angle;
    if angle <= 0.0 {
        angle + 2.0 * PI
    } else if angle > 2.0 * PI {
        angle - 2.0 * PI
    } else {
        angle
    }
}

type MyLine = ((i32, i32), (i32, i32));


fn ccw(a: (i32, i32), b: (i32, i32), c: (i32, i32)) -> bool {
    let (ax, ay) = a;
    let (bx, by) = b;
    let (cx, cy) = c;
    (bx - ax) * (cy - ay) - (cx - ax) * (by - ay) > 0
}

fn line_intersects_line(a: MyLine, b: MyLine) -> bool {
    // let (a1, a2) = a;
    // let (b1, b2) = b;
    // let (a1x, a1y) = a1;
    // let (a2x, a2y) = a2;
    // let (b1x, b1y) = b1;
    // let (b2x, b2y) = b2;
    //
    // let d = (a1x - a2x) * (b1y - b2y) - (a1y - a2y) * (b1x - b2x);
    // let d = d as f32;
    // if d == 0.0 {
    //     return false;
    // }
    //
    // let ua = ((b1x - b2x) * (a1y - b2y) - (b1y - b2y) * (a1x - b2x)) as f32 / d;
    // let ub = ((a1x - a2x) * (b1y - a2y) - (a1y - a2y) * (b1x - a2x)) as f32 / d;
    //
    // ua >= 0.0 && ua <= 1.0 && ub >= 0.0 && ub <= 1.0

    // check with ccw

    // ccw(a.0, a.1, b.0) != ccw(a.0, a.1, b.1) && ccw(a.0, b.0, b.1) != ccw(a.1, b.0, b.1)
    use line_intersection::{LineInterval, LineRelation};
    use geo::{Coordinate, Line, Point};

    let first = LineInterval::line_segment(Line {
        start: (a.0.0 as f32, a.0.1 as f32).into(),
        end: (a.1.0 as f32, a.1.1 as f32).into(),
    });

    let second = LineInterval::line_segment(Line {
        start: (b.0.0 as f32, b.0.1 as f32).into(),
        end: (b.1.0 as f32, b.1.1 as f32).into(),
    });

    match first.relate(&second) {
        LineRelation::DivergentDisjoint => false,
        LineRelation::DivergentIntersecting(p) => {
            if p.x().fract() != 0.0 {
                return true;
            }
            if p.y().fract() != 0.0 {
                return true;
            }
            let ix = p.x() as i32;
            let iy = p.y() as i32;

            // check for all four endpoints whether they are this (ix, iy) point
            if (ix, iy) == a.0 || (ix, iy) == a.1 || (ix, iy) == b.0 || (ix, iy) == b.1 {
                return false;
            }

            true
        },
        _ => false,
    }
}

fn check_line_intersects_any_other_line(line: MyLine, lines: &[MyLine]) -> bool {
    for &other in lines {
        if other != line && line_intersects_line(line, other) {
            return true;
        }
    }
    false
}

fn get_lines_from_sequence_of_points(points: &[(i32, i32)]) -> Vec<MyLine> {
    let mut lines = Vec::new();
    for i in 0..points.len() - 1 {
        lines.push((points[i], points[i + 1]));
    }
    lines
}

// fn angle_between_directions(old: (f32, f32), new: (f32, f32)) -> f32 {
//     let old_angle = angle_between((0.0, 0.0), old);
//     let new_angle = angle_between((0.0, 0.0), new);
//     old_angle - new_angle
// }

fn is_point_left_of_line(a: (i32, i32), b: (i32, i32), p: (i32, i32)) -> bool {
    let dx = (b.0 - a.0) as f32;
    let dy = (b.1 - a.1) as f32;
    let px = (p.0 - a.0) as f32;
    let py = (p.1 - a.1) as f32;
    px * dy - py * dx < 0.0
}

const DIST: f32 = 8.0;


fn skiuswrap2(mut points: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut outline = vec![];

    points.sort_by_key(|&(x, y)| x);
    points.dedup();

    let mut start = points[0];
    points.remove(0);
    outline.push(start);
    let first = start;
    let mut old = (start.0, start.1 - 1);

    // Jarvis Wrap
    while !points.is_empty() {
        // let mut candidates = find_points_within_distance(curr, DIST, &points);
        // if candidates.is_empty() {
        //     candidates = points.to_vec();
        // }
        let mut best = None;
        let mut best_angle = 3.0 * PI;
        for (idx, &end@(x, y)) in points.clone().iter().enumerate() {
            println!("Checking point {}: {:?}", idx, end);
            if !is_point_within_dist(start, end, DIST) {
                println!("Point {} is too far away", idx);
                continue;
            }
            println!("Point {} is within distance", idx);

            // Problem is here, the new point might actually be soo far back, it is already within the polygon
            // could fix by actually storing outline lines, and checking if adding that outline would cause an intersection

            if outline.len() > 1 {
                if check_line_intersects_any_other_line((start, end), &get_lines_from_sequence_of_points(&outline)) {
                    println!("Point {} is intersecting", idx);
                    continue;
                }
            }

            let angle = angle_between_three_points(old, start, end);
            println!("Checking angle: {}", angle * 180.0 / PI);
            if angle < best_angle {
                best = Some(idx);
                best_angle = angle;
            }
        }
        if let Some(idx) = best {
            println!("Best angle: {}", best_angle * 180.0 / PI);
            println!("");

            // if first is better than this one, we're done
            if is_point_within_dist(first, points[idx], DIST) {
                let angle = angle_between_three_points(start, points[idx], first);
                if angle < best_angle {
                    // reached the end
                    outline.push(points[idx]);
                    println!("Reached beginning again");
                    break;
                }
            }
            old = start;
            start = points.remove(idx);
            outline.push(start);
        } else {
            // TODO: fix
            println!("No candidate");
            break;
        }
    }

    outline
}

// returns the outline
// fn skiuswrap(mut points: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
//     let mut outline = vec![];
//
//     points.sort_by_key(|&(x, y)| x);
//
//     let first = points[0];
//     let mut last = points.remove(0);
//     outline.push(last);
//     outline.push(last);
//     let mut last_angle = 0.5 * PI;
//
//     // Jarvis Wrap
//     while !points.is_empty() {
//         // let mut candidates = find_points_within_distance(curr, 5.0, &points);
//         // if candidates.is_empty() {
//         //     candidates = points.to_vec();
//         // }
//         let mut best = None;
//         let mut best_angle = 2.0 * PI;
//         for (idx, &(x, y)) in points.clone().iter().enumerate() {
//             if !is_point_within_dist(last, (x, y), DIST) {
//                 continue;
//             }
//
//             let angle = angle_from_to(last, (x, y));
//             // let mut angle = angle - last_angle;
//             let mut angle = angle - angle_from_to(last, outline[outline.len() - 2]);
//             if angle < 0.0 {
//                 angle += 2.0 * PI;
//             }
//             if angle < best_angle {
//                 best = Some(idx);
//                 best_angle = angle;
//             }
//         }
//         if let Some(idx) = best {
//             println!("Best angle: {}", best_angle * 180.0 / PI);
//
//             // if first is better than this one, we're done
//             if is_point_within_dist(first, points[idx], DIST) {
//                 let angle = angle_from_to(last, first);
//                 let mut angle = angle - last_angle;
//                 if angle < 0.0 {
//                     angle += 2.0 * PI;
//                 }
//                 if angle < best_angle {
//                     // reached the end
//                     break;
//                 }
//             }
//
//             last = points.remove(idx);
//             outline.push(last);
//             last_angle = angle_from_to(outline[outline.len() - 2], last);
//         } else {
//             // TODO: fix
//             break;
//         }
//     }
//
//     outline
// }

fn skiuswrap(mut points: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut outline = vec![];

    points.sort_by_key(|&(x, y)| x);

    let mut start = points[0];

    let mut i = 0;

    loop {
        outline.push(start);
        let mut candidates = find_points_within_distance(start, DIST, &points);
        candidates = candidates.into_iter().filter(|p| !outline.contains(p)).collect();
        candidates.push(outline[0]);
        if candidates.len() < 1 {
            println!("No candidates");
            break;
            // candidates = points.clone();
        }
        let mut end = candidates[0];
        // if start == end {
        //     end = candidates[1];
        // }
        for j in 0..candidates.len() {
            if (end == start) || is_point_left_of_line(start, end, candidates[j]) {
                end = candidates[j];
            }
        }
        start = end;
        if end == outline[0] {
            println!("Done, found beginning");
            break;
        }
        i += 1;
        if i > 200 {
            break;
        }
    }



    outline
}


fn gen_img(points: &[(i32, i32)], outline: &[(i32, i32)], name: &str) {
    let root_area = BitMapBackend::new(name, (1000, 1000))
        .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Scatter Demo", ("sans-serif", 40))
        .build_cartesian_2d(MIN..MAX, MIN..MAX)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        points.iter().map(|point| TriangleMarker::new(*point, 5, &BLUE)),
    ).unwrap();
    ctx.draw_series(outline.iter().map(|point| Circle::new(*point, 5, &RED)))
        .unwrap();
}

fn main() {

    let points1 = get_sample_points(50, (-15.0, 15.0));
    let points2 = get_sample_points(50, (0.0, 0.0));
    let points3 = get_sample_points(50, (20.0, 10.0));
    let points: Vec<_> = points1.iter().chain(points2.iter()).chain(points3.iter()).cloned().collect();
    let outline_points = skiuswrap2(points.clone());

    for i in 0..outline_points.len() {
        gen_img(&points, &outline_points[0..=i], &format!("outline_{}.png", i));
    }
}
