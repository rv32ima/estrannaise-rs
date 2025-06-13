use estrannaise_rs::models::e2::*;
use textplots::{Chart, Plot};

fn main() {
    let dose = Dose::from_f32(6.0);
    let model = Model::from_type(Type::EEnim);
    let dosing_interval = 10.0; // 10.0 days per dose

    println!("{:} {:}mg/{:}d", model.to_str(), dose.to_str(), dosing_interval);

    Chart::new(150, 80, 0.0, 30.0)
        .lineplot(&textplots::Shape::Continuous(Box::new(|x| {
            model.simulate(&dose, x, dosing_interval)
        })))
        .display();
}

fn fill_curve<F>(f: F, xmin: f32, xmax: f32, nsteps: f32) -> Vec<(f32, f32)>
where
    F: Fn(f32) -> f32,
{
    let mut ret = Vec::new();
    let mut x = xmin;
    while x <= xmax {
        let point = f(x);
        ret.push((x, point));
        x += (xmax - xmin) / (nsteps - 1.0)
    }

    ret
}

fn format_time(n: f32) -> String {
    let days = n.floor();
    let hours = f32::round((n - days) * 24.0);

    return format!("{:?}d {:}h", days, hours);
}
