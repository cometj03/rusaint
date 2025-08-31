use crate::application::USaintClient;
use wdpe::command::element::action::ButtonPressEventCommand;
use wdpe::command::WebDynproCommandExecutor;
use wdpe::command::element::complex::{
    SapTableBodyCommand, SapTableLSDataCommand, SapTableVerticalScrollEventCommand,
};
use wdpe::element::complex::SapTableDef;
use wdpe::element::complex::sap_table::FromSapTable;
use wdpe::element::definition::ElementDefinition;
use wdpe::element::parser::ElementParser;
use wdpe::error::{ElementError, WebDynproError};

pub(crate) async fn try_table_into_with_scroll<T: for<'body> FromSapTable<'body>>(
    client: &mut USaintClient,
    mut parser: ElementParser,
    table: SapTableDef,
) -> Result<Vec<T>, WebDynproError> {
    let row_count = parser
        .read(SapTableLSDataCommand::new(table.clone()))?
        .row_count()
        .map(|u| u.to_owned())
        .ok_or_else(|| ElementError::NoSuchData {
            element: table.clone().id().to_string(),
            field: "row_count".to_string(),
        })?
        .try_into()
        .unwrap();
    let mut table_body = parser.read(SapTableBodyCommand::new(table.clone()))?;
    let mut results: Vec<T> = Vec::with_capacity(row_count);
    while results.len() < row_count {
        let mut partial_results = table_body.try_table_into::<T>(&parser)?;
        if results.len() + partial_results.len() > row_count {
            let overflowed = results.len() + partial_results.len() - row_count;
            partial_results.drain(0..overflowed);
        }
        results.append(&mut partial_results);
        if results.len() < row_count {
            let event = parser.read(SapTableVerticalScrollEventCommand::new(
                table.clone(),
                results.len().try_into().unwrap(),
                "",
                "SCROLLBAR",
                false,
                false,
                false,
                false,
            ))?;
            client.process_event(false, event).await?;
            parser = ElementParser::new(client.body());
            table_body = parser.read(SapTableBodyCommand::new(table.clone()))?;
        }
    }
    Ok(results)
}

use crate::application::course_schedule::model::Lecture;

pub(crate) async fn try_table_into_lecture_with_scroll(
    client: &mut USaintClient,
    mut parser: ElementParser,
    table: SapTableDef,
) -> Result<Vec<Lecture>, WebDynproError> {
    let row_count = parser
        .read(SapTableLSDataCommand::new(table.clone()))?
        .row_count()
        .map(|u| u.to_owned())
        .ok_or_else(|| ElementError::NoSuchData {
            element: table.clone().id().to_string(),
            field: "row_count".to_string(),
        })?
        .try_into()
        .unwrap();
    let mut table_body = parser.read(SapTableBodyCommand::new(table.clone()))?;
    let mut results: Vec<Lecture> = Vec::with_capacity(row_count);
    while results.len() < row_count {
        println!("results length: {}", results.len());
        let partial_results: Vec<Lecture> = table_body.try_table_into::<Lecture>(&parser)?;
        let mut replaced_results: Vec<Lecture> = Vec::with_capacity(partial_results.len());

        for lecture in &partial_results {
            let lec = match lecture.syllabus {
                Some(ref id) => {
                    let url = click_syllabus_button_by_id(id.to_string(), client, &parser).await?;
                    lecture.clone().replace_syllabus(url)
                }
                None => lecture.clone()
            };
            replaced_results.push(lec);
        }

        if results.len() + replaced_results.len() > row_count {
            let overflowed = results.len() + replaced_results.len() - row_count;
            replaced_results.drain(0..overflowed);
        }
        results.append(&mut replaced_results);
        if results.len() < row_count {
            let event = parser.read(SapTableVerticalScrollEventCommand::new(
                table.clone(),
                results.len().try_into().unwrap(),
                "",
                "SCROLLBAR",
                false,
                false,
                false,
                false,
            ))?;
            client.process_event(false, event).await?;
            parser = ElementParser::new(client.body());
            table_body = parser.read(SapTableBodyCommand::new(table.clone()))?;
        }
    }
    Ok(results)
}

use wdpe::element::action::ButtonDef;

async fn click_syllabus_button_by_id(
    id: String,
    client: &mut USaintClient,
    parser: &ElementParser,
) -> Result<String, WebDynproError> {
    let syllabus_btn = ButtonDef::new_dynamic(id);
    let btn_press = parser.read(ButtonPressEventCommand::new(syllabus_btn))?;
    let res = client.process_event_get_response(btn_press).await?;
    let re = regex_lite::Regex::new(r#".*"url":"([^"]*)".*"#).unwrap();
    let (_, [url]) = re.captures(&res).unwrap().extract();
    let url = url
        .replace("\\x3a", ":")
        .replace("\\x2f", "/")
        .replace("\\x3f", "?")
        .replace("\\x3d", "=")
        .replace("\\x26", "&")
        .replace("\\x2a", "*");

    Ok(url.to_string())
}

#[test]
fn test_regex() {
    let s = r#"<updates><delta-update windowid="sapwd_main_window"><script-call><![CDATA[application.exec("setFocus",{});]]></script-call><script-call><![CDATA[application.exec("unlock",{});]]></script-call><script-call><![CDATA[application.exec("setMessagesAsJson",{"messages":"\x5b\x5d"});]]></script-call><script-call><![CDATA[application.exec("openExternalWindow",{"windowId":"sapwd_main_window","url":"https\x3a\x2f\x2foffice.ssu.ac.kr\x2foz70\x2fozView.jsp\x3fozrname\x3dzcm_get_abeek_plan_2018_new\x26category\x3dCM\x26cnt\x3d5\x26pName\x3darg1,arg2,arg3,UNAME,P_RANDOM\x26pValue\x3d2025,090,22211900,OZASPN,\x2a19468","target":"","location":"0","menubar":"0","resizable":"1","scrollbars":"0","status":"0","toolbar":"0","width":"","height":"","left":"0","top":"0","usePost":"0","postParameters":""});]]></script-call><initialize-ids><![CDATA[<span id="UCF_InitializeIDs" data-sap-ls-style="display:none">[]</span>]]></initialize-ids></delta-update></updates>"#;
    let re = regex_lite::Regex::new(r#".*"url":"([^"]*)".*"#).unwrap();
    let (_, [url]) = re.captures(s).unwrap().extract();
    println!("url: {}", url);
}

