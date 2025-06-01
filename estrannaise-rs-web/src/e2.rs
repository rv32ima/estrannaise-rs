use wasm_bindgen::prelude::*;
use estrannaise_rs::models::e2::*;

#[wasm_bindgen(js_name = Type)]
pub struct TypeWrapper(Type);

#[wasm_bindgen(js_class = Type)]
impl TypeWrapper {
  pub fn from_str(s: &str) -> TypeWrapper {
    TypeWrapper(Type::from_str(s))
  }
}

#[wasm_bindgen(js_name = Dose)]
pub struct DoseWrapper(Dose);

#[wasm_bindgen(js_class = Dose)]
impl DoseWrapper {
  pub fn from_f32(f: f32) -> DoseWrapper {
    DoseWrapper(Dose::from_f32(f))
  }

  pub fn with_conversion_factor(&self, cf: f32) -> DoseWrapper {
    DoseWrapper(self.0.with_conversion_factor(cf))
  }

  pub fn converted_dose(&self) -> f32 {
    self.0.converted_dose()
  }
}

#[wasm_bindgen(js_name = Model)]
pub struct ModelWrapper(Model);

#[wasm_bindgen(js_class = Model)]
impl ModelWrapper {
    pub fn to_str(&self) -> String {
      self.0.to_str()
    }

    pub fn from_str(s: &str) -> ModelWrapper {
      let ty = Type::from_str(s);
      ModelWrapper(Model::from_type(ty))
    }

    pub fn from_type(t: TypeWrapper) -> ModelWrapper {
      ModelWrapper(Model::from_type(t.0))
    }

    pub fn simulate(&self, dose: &DoseWrapper, t: f32, steady_state: bool, dosing_interval: f32) -> f32 {
      self.0.simulate(&dose.0, t, steady_state, dosing_interval)
    }

    pub fn simulate_rand(&self, dose: &DoseWrapper, t: f32, steady_state: bool, dosing_interval: f32) -> f32 {
      self.0.simulate_rand(&dose.0, t, steady_state, dosing_interval)
    }

} 
