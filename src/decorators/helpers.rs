use std::{collections::HashMap,collections::HashSet};
use xml_oxide::sax::StartElement;

pub fn attributes_to_dict(attr_dict : &mut HashMap<String,String>, el: &StartElement, attr_names : HashSet::<&str>) {
  attr_dict.clear();
  for el_attr in el.attributes() {
    if attr_names.contains(el_attr.name) {
      attr_dict.insert(el_attr.name.to_string(),el_attr.value.to_string());
    }
  }
}