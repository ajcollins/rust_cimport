use crate::dimensions::valuetype::ValueType;
use crate::dimensions::role::Role;
use crate::dimensions::asset::Asset;
use crate::dimensions::vulnerability::Vulnerability;
use crate::dimensions::projectsettings::ProjectSettings;
use crate::dimensions::environment::Environment;

pub struct SavedState {
  pub tv_types : Option<Box<Vec<ValueType>>>,
  pub dv_types : Option<Box<Vec<ValueType>>>,
  pub p_settings : Option<Box<ProjectSettings>>,
  pub environments : Option<Box<Vec<Environment>>>,
  pub roles : Option<Box<Vec<Role>>>,
  pub assets : Option<Box<Vec<Asset>>>,
  pub vulnerabilities : Option<Box<Vec<Vulnerability>>>
}

impl SavedState {
  pub fn new() -> SavedState {
    SavedState {
      tv_types : None,
      dv_types : None,
      p_settings : None,
      environments : None,
      roles : None,
      assets : None,
      vulnerabilities : None
    }
  }
}