use xml_oxide::{sax::StartElement,sax::EndElement};
use crate::decorators::savedstate::SavedState;

pub trait ParseDecorator {
  fn parse_start_element(&mut self, _el: &StartElement){}
  fn parse_characters(&mut self, _data : &str){}
  fn parse_end_element(&mut self, _el: &EndElement){}
  fn save_state(&self, _ss: &mut SavedState){}
}
