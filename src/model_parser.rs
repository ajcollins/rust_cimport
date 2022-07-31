use std::{fs::File};
use xml_oxide::{sax::parser::Parser, sax::Event};
use crate::decorators::{parsedecorator::ParseDecorator,tvtypes::TVTypesHandler,domainvalues::DomainValuesHandler, projectsettings::ProjectSettingsHandler, riskanalysis::RiskAnalysisHandler, savedstate::SavedState};

pub struct ModelParser {
  pub state : SavedState,
  decorator : Option<Box<dyn ParseDecorator>>
}

impl ModelParser {
  pub fn new() -> ModelParser {
    ModelParser {
      state : SavedState::new(),
      decorator : None
    }
  }

  pub fn parse(&mut self, model_file : &String) {

    let f = File::open(model_file).unwrap();
    let mut p = Parser::from_reader(f);

    loop {
      let res = p.read_event();

      match res {
        Ok(event) => match event {
          Event::StartDocument => {}
          Event::EndDocument => {
            break;
          }
          Event::StartElement(el) => {
            if !el.is_empty {
              if el.name == "tvtypes" {
                self.decorator = Some(Box::new(TVTypesHandler::new()));
              }
              if el.name == "domainvalues" {
                self.decorator = Some(Box::new(DomainValuesHandler::new()));
              }
              if el.name == "cairis" {
                self.decorator = Some(Box::new(ProjectSettingsHandler::new()));
              }
              if el.name == "riskanalysis" {
                self.decorator = Some(Box::new(RiskAnalysisHandler::new()));
              }
              else if let Some(d) = &mut self.decorator {
                d.parse_start_element(&el);
              }
            }
          }
          Event::EndElement(el) => {
            if el.name == "feed" {
              break;
            }
            else if el.name == "tvtypes" || el.name == "domainvalues" || el.name == "cairis" || el.name == "riskanalysis" {
              if let Some(d) = &mut self.decorator {
                d.save_state(&mut self.state);
                self.decorator = None;
              }
            } 
            else if let Some(d) = &mut self.decorator {
              d.parse_end_element(&el);
            }
          }
          Event::Characters(data) => {
            if let Some(d) = &mut self.decorator {
              d.parse_characters(&data);
            }
          }
          Event::Reference(_) => {}
           _ => {}
        },
        Err(err) => {
          println!("{}", err);
          break;
        }
      } 
    }
  }
}
