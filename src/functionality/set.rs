use std::collections::HashMap;

use rsheet_lib::{cell_expr::{CellArgument, CellExpr}, cell_value::CellValue, command::CellIdentifier, cells::column_name_to_number};
use crate::functionality::get::get;

pub fn set(spreadsheet: &mut HashMap<CellIdentifier, CellValue>, cell_identifier: CellIdentifier, expression: CellExpr) -> CellValue {
  let variables = expression.find_variable_names();
  let mut map: HashMap<String, CellArgument> = HashMap::new();
  for var in variables {
    match var.contains("_") {
      true => {
        let parts: Vec<&str> = var.split('_').collect();

        let (col_start, r_start): (String, String) = parts[0].chars()
          .partition(|c| c.is_alphabetic());
        let (col_end, r_end): (String, String) = parts[1].chars()
          .partition(|c| c.is_alphabetic());

        let row_start: u32 = r_start.parse().unwrap();
        let row_end: u32 = r_end.parse().unwrap();

        // vertical vector A1_A10
        if col_start == col_end  {
          let mut vec: Vec<CellValue> = Vec::new();

          for i in row_start..=row_end {
            let cell_id = CellIdentifier { col: column_name_to_number(&col_start), row: i - 1 };
            let val = get(spreadsheet, cell_id);

            vec.push(val);
          }

          map.insert(var.to_string(), CellArgument::Vector(vec));

        // horizontal vector A1_F1
        } else if row_start == row_end {
          let mut vec: Vec<CellValue> = Vec::new();

          let start = col_start.chars().next().unwrap();
          let end = col_end.chars().next().unwrap();

          for c in start..=end {
            let cell_id = CellIdentifier { col: column_name_to_number(&c.to_string()), row: row_start - 1 };
            let val = get(spreadsheet, cell_id);

            vec.push(val);
          }

          map.insert(var.to_string(), CellArgument::Vector(vec));

        // matrix A1_F10
        } else {
          let mut mat: Vec<Vec<CellValue>> = Vec::new();

          let c_start = col_start.chars().next().unwrap();
          let c_end = col_end.chars().next().unwrap();

          for c in c_start..=c_end {
            let mut vec: Vec<CellValue> = Vec::new();
            
            for i in row_start..=row_end {
              let cell_id = CellIdentifier { col: column_name_to_number(&c.to_string()), row: i - 1 };
              let val = get(spreadsheet, cell_id);
              
              vec.push(val);
            }

            mat.push(vec);
          }

          map.insert(var.to_string(), CellArgument::Matrix(mat));
        }
      },
      false => {
        let cell_id = CellIdentifier { col: column_name_to_number(&var.chars().nth(0).unwrap().to_string()), row: var.chars().nth(1).unwrap() as u32 - '0' as u32 - 1 };
        map.insert(var.to_string(), CellArgument::Value(get(spreadsheet, cell_id)));
      },
    }
  }
  
  let result = match expression.evaluate(&map) {
    Ok(value) => {
      value
    },
    Err(error) => {
      CellValue::Error(format!("{:?}", error))
    }
  };
  spreadsheet.insert(cell_identifier, result.clone());
  result
}