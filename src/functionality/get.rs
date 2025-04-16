use std::collections::HashMap;

use rsheet_lib::{cell_value::CellValue, command::CellIdentifier};

pub fn get(spreadsheet: &HashMap<CellIdentifier, CellValue>, cell_identifier: CellIdentifier) -> CellValue {
  if !spreadsheet.contains_key(&cell_identifier) {
    return CellValue::None;
  }
  let res = match &spreadsheet[&cell_identifier] {
    CellValue::Int(value) => {
      CellValue::Int(*value)
    },
    CellValue::String(value) => {
      CellValue::String(value.clone())
    },
    CellValue::Error(value) => {
      CellValue::Error(value.clone())
    },
    CellValue::None => {
      CellValue::None
    }
  };
  return res;
}