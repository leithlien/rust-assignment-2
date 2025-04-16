use rsheet_lib::cell_expr::CellExpr;
use rsheet_lib::cell_value::CellValue;
use rsheet_lib::command::{CellIdentifier, Command};
use rsheet_lib::connect::{
    Connection, Manager, ReadMessageResult, Reader, WriteMessageResult, Writer,
};
use rsheet_lib::replies::Reply;

use std::cell::Cell;
use std::error::Error;

use log::info;

use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use rsheet_lib::cells::column_number_to_name;
use crate::functionality::get::get;
use crate::functionality::set::set;

pub fn handle_command(
  msg: String,
  mut recv: &mut dyn Reader, 
  mut send: &mut dyn Writer, 
  spreadsheet: &Arc<Mutex<HashMap<CellIdentifier, CellValue>>>
) -> WriteMessageResult {
  // rsheet_lib already contains a FromStr<Command> (i.e. parse::<Command>)
    // implementation for parsing the get and set commands. This is just a
    // demonstration of how to use msg.parse::<Command>, you may want/have to
    // change this code.
    let mut sheet = spreadsheet.lock().unwrap();
    let mut command_set = false;
    let reply = match msg.parse::<Command>() {
        Ok(command) => match command {
            Command::Get { cell_identifier } => {
              let id = format!("{}{}", column_number_to_name(cell_identifier.col), cell_identifier.row + 1);
              let value = get(&sheet, cell_identifier);
              Reply::Value(id, value)
            },
            Command::Set {
                cell_identifier,
                cell_expr,
            } => {
                command_set = true;
                let id = format!("{}{}", column_number_to_name(cell_identifier.col), cell_identifier.row + 1);
                let expression = CellExpr::new(&cell_expr);
                let value = set(&mut sheet, cell_identifier, expression);
                Reply::Value(id, value)
            },
        },
        Err(e) => Reply::Error(e),
    };

    if !command_set {
      match send.write_message(reply) {
        WriteMessageResult::Ok => {
            // Message successfully sent, continue.
            return WriteMessageResult::Ok;
        }
        WriteMessageResult::ConnectionClosed => {
            // The connection was closed. This is not an error, but
            // should terminate this connection.
            return WriteMessageResult::ConnectionClosed;
        }
        WriteMessageResult::Err(e) => {
            // An unexpected error was encountered.
            //return Err(Box::new(e));
            return WriteMessageResult::Err(e);
        }
      }
    }
    WriteMessageResult::Ok
}