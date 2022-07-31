use crate::dimensions::valuetype::ValueType;
use crate::dimensions::role::Role;
use crate::dimensions::asset::Asset;
use crate::dimensions::projectsettings::ProjectSettings;

pub struct SavedState {
  pub tv_types : Option<Box<Vec<ValueType>>>,
  pub dv_types : Option<Box<Vec<ValueType>>>,
  pub p_settings : Option<Box<ProjectSettings>>,
  pub roles : Option<Box<Vec<Role>>>,
  pub assets : Option<Box<Vec<Asset>>>
}

impl SavedState {
  pub fn new() -> SavedState {
    SavedState {
      tv_types : None,
      dv_types : None,
      p_settings : None,
      roles : None,
      assets : None
    }
  }
}