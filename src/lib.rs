use evtx::xml_to_json;
use win_event_log::prelude::*;
pub fn get_events(query: &str) -> anyhow::Result<Vec<String>> {
    match WinEvents::get(query) {
        Ok(events) => Ok(events
            .into_iter()
            .map(|xml| xml_to_json(xml.to_string().as_str()))
            .collect::<Vec<String>>()),
        Err(e) => Err(anyhow::Error::msg(e)),
    }
}
