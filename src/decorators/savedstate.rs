use cairis_core::dimensions::valuetype::ValueType;
use cairis_core::dimensions::role::Role;
use cairis_core::dimensions::asset::Asset;
use cairis_core::dimensions::vulnerability::Vulnerability;
use cairis_core::dimensions::attacker::Attacker;
use cairis_core::dimensions::projectsettings::ProjectSettings;
use cairis_core::dimensions::environment::Environment;

pub struct SavedState {
  pub tv_types : Option<Box<Vec<ValueType>>>,
  pub dv_types : Option<Box<Vec<ValueType>>>,
  pub p_settings : Option<Box<ProjectSettings>>,
  pub environments : Option<Box<Vec<Environment>>>,
  pub roles : Option<Box<Vec<Role>>>,
  pub assets : Option<Box<Vec<Asset>>>,
  pub vulnerabilities : Option<Box<Vec<Vulnerability>>>,
  pub attackers : Option<Box<Vec<Attacker>>>
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
      vulnerabilities : None,
      attackers : None
    }
  }
}
