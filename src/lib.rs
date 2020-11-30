use evtx::xml_to_json;
use serde_json::{from_str, Value};
use win_event_log::prelude::*;
pub fn get_events(query: &str) -> anyhow::Result<Vec<Value>> {
    match WinEvents::get(query) {
        Ok(events) => Ok(events
            .into_iter()
            .flat_map(|xml| {
                from_str(&xml_to_json(
                    xml.to_string().as_str().trim_matches(char::from(0)),
                ))
            })
            .collect::<Vec<Value>>()),
        Err(e) => Err(anyhow::Error::msg(e)),
    }
}
mod tests {
    #[test]
    fn test_application() {
        use super::*;
        let query = format!(
            "<QueryList>
               <Query Id=\"0\" Path='Application'>
                   <Select Path='Application'>
                   *[System[TimeCreated[
                       @SystemTime&gt;='2020-11-29T10:09:35.5308139Z' and 
                       @SystemTime&lt;='2020-11-30T10:09:35.5308139Z']]]
                   </Select>
               </Query>
         </QueryList>"
        );
        match get_events(&query) {
            Ok(events) => println!("{:#?}", events),
            Err(e) => println!("{:#?}", e),
        }
    }
    #[test]
    fn test_system() {
        use super::*;
        let query = format!(
            "<QueryList>
               <Query Id=\"0\" Path='System'>
                   <Select Path='System'>
                   *[System[TimeCreated[
                       @SystemTime&gt;='2020-11-29T10:09:35.5308139Z' and 
                       @SystemTime&lt;='2020-11-30T10:09:35.5308139Z']]]
                   </Select>
               </Query>
         </QueryList>"
        );
        match get_events(&query) {
            Ok(events) => println!("{:#?}", events),
            Err(e) => println!("{:#?}", e),
        }
    }
}
