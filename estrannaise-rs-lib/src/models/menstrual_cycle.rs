use lazy_static::lazy_static;
use splines::*;

enum Type {
    E2,
    E2P5,
    E2P95,
}

const fn menstrual_cycle_data(t: Type) -> [(u8, f32); 30] {
    match t {
        Type::E2 => [
            (0, 37.99),
            (1, 40.59),
            (2, 37.49),
            (3, 34.99),
            (4, 35.49),
            (5, 39.54),
            (6, 41.99),
            (7, 44.34),
            (8, 53.43),
            (9, 58.58),
            (10, 71.43),
            (11, 98.92),
            (12, 132.31),
            (13, 177.35),
            (14, 255.88),
            (15, 182.80),
            (16, 85.23),
            (17, 70.98),
            (18, 87.97),
            (19, 109.92),
            (20, 122.77),
            (21, 132.56),
            (22, 150.30),
            (23, 133.81),
            (24, 137.16),
            (25, 134.96),
            (26, 92.73),
            (27, 85.68),
            (28, 46.34),
            (29, 41.19),
        ],
        Type::E2P5 => [
            (0, 15.68),
            (1, 17.99),
            (2, 20.48),
            (3, 21.63),
            (4, 22.60),
            (5, 23.86),
            (6, 25.44),
            (7, 30.64),
            (8, 33.96),
            (9, 42.95),
            (10, 51.88),
            (11, 50.79),
            (12, 65.79),
            (13, 91.89),
            (14, 137.25),
            (15, 131.30),
            (16, 43.55),
            (17, 42.12),
            (18, 56.83),
            (19, 73.49),
            (20, 79.70),
            (21, 72.75),
            (22, 79.46),
            (23, 76.79),
            (24, 76.05),
            (25, 80.22),
            (26, 57.26),
            (27, 47.62),
            (28, 27.77),
            (29, 25.60),
        ],
        Type::E2P95 => [
            (0, 52.97),
            (1, 51.12),
            (2, 51.58),
            (3, 54.74),
            (4, 53.59),
            (5, 57.08),
            (6, 61.20),
            (7, 60.16),
            (8, 72.79),
            (9, 85.36),
            (10, 94.46),
            (11, 133.70),
            (12, 218.89),
            (13, 314.28),
            (14, 413.41),
            (15, 388.28),
            (16, 140.11),
            (17, 108.52),
            (18, 135.06),
            (19, 181.42),
            (20, 191.73),
            (21, 196.05),
            (22, 189.45),
            (23, 195.64),
            (24, 208.23),
            (25, 219.75),
            (26, 174.38),
            (27, 148.77),
            (28, 135.58),
            (29, 188.92),
        ],
    }
}

fn menstrual_cycle_spline(t: Type) -> Spline<f32, f32> {
    let data = menstrual_cycle_data(t);
    let mut points = Vec::new();

    for (i, point) in data.iter().enumerate() {
        match i {
            0 => points.push(Key::new(point.0 as f32, point.1, Interpolation::Linear)),
            _ => points.push(Key::new(point.0 as f32, point.1, Interpolation::CatmullRom)),
        }
    }

    Spline::<f32, f32>::from_vec(points)
}

lazy_static! {
    static ref MENSTRUAL_CYCLE_SPLINE: Spline<f32, f32> =
        menstrual_cycle_spline(Type::E2);
    static ref MENSTRUAL_CYCLE_SPLINE_P05: Spline<f32, f32> =
        menstrual_cycle_spline(Type::E2P5);
    static ref MENSTRUAL_CYCLE_SPLINE_P95: Spline<f32, f32> =
        menstrual_cycle_spline(Type::E2P95);
}

#[derive(Copy, Clone)]
pub struct MenstrualCycleCurvePoint {
    pub Time: f32,
    pub E2: f32,
    pub E2p5: f32,
    pub E2p95: f32,
}

pub fn fill_menstrual_cycle_curve(
    xmin: f32,
    xmax: f32,
    nbsteps: i32,
    conversion_factor: f32,
) -> Vec<MenstrualCycleCurvePoint> {
    let mut points = Vec::new();
    let mut t = xmin;
    while t <= xmax {
        let curve_point = ((t % 28.0) + 28.0) % 28.0;
        points.push(MenstrualCycleCurvePoint {
            Time: t,
            E2: conversion_factor
                * MENSTRUAL_CYCLE_SPLINE
                    .clamped_sample(curve_point)
                    .unwrap_or_default(),
            E2p5: conversion_factor
                * MENSTRUAL_CYCLE_SPLINE_P05
                    .clamped_sample(curve_point)
                    .unwrap_or_default(),
            E2p95: conversion_factor
                * MENSTRUAL_CYCLE_SPLINE_P95
                    .clamped_sample(curve_point)
                    .unwrap_or_default(),
        });
        t = t + ((xmax - xmin) / (nbsteps as f32 - 1.0));
    }

    points
}
