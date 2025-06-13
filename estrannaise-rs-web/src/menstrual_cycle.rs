#![allow(non_snake_case)]

use estrannaise_rs::models::menstrual_cycle::{self, *};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = MenstrualCycleCurvePoint)]
pub struct MenstrualCycleCurvePointWrapper(MenstrualCycleCurvePoint);

#[wasm_bindgen(js_class = MenstrualCycleCurvePoint)]
impl MenstrualCycleCurvePointWrapper {
  #[wasm_bindgen(getter)]
  pub fn Time(&self) -> f32 {
    self.0.time
  }

  #[wasm_bindgen(getter)]
  pub fn E2(&self) -> f32 {
    self.0.e2
  }

  #[wasm_bindgen(getter)]
  pub fn E2p5(&self) -> f32 {
    self.0.e2p5
  }

  #[wasm_bindgen(getter)]
  pub fn E2p95(&self) -> f32 {
    self.0.e2p95
  }
}

#[wasm_bindgen]
pub fn fill_menstrual_cycle_curve(
    xmin: f32,
    xmax: f32,
    nbsteps: i32,
    conversion_factor: f32,
) -> Vec<MenstrualCycleCurvePointWrapper> {
    menstrual_cycle::fill_menstrual_cycle_curve(xmin, xmax, nbsteps, conversion_factor)
        .iter()
        .map(|x| MenstrualCycleCurvePointWrapper(x.clone()))
        .collect()
}
