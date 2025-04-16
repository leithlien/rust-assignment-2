use rsheet_lib::cell_expr::CellExpr;
use rsheet_lib::cell_value::CellValue;
use rsheet_lib::command::{CellIdentifier, Command};
use rsheet_lib::connect::{
    Connection, Manager, ReadMessageResult, Reader, WriteMessageResult, Writer,
};
use rsheet_lib::replies::Reply;

//use std::cell::Cell;
use std::error::Error;

use log::info;

mod functionality;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use rsheet_lib::cells::column_number_to_name;

pub fn start_server<M>(mut manager: M) -> Result<(), Box<dyn Error>>
where
    M: Manager,
{
    // This initiates a single client connection, and reads and writes messages
    // indefinitely.
    let (mut recv, mut send) = match manager.accept_new_connection() {
        Connection::NewConnection { reader, writer } => (reader, writer),
        Connection::NoMoreConnections => {
            // There are no more new connections to accept.
            return Ok(());
        }
    };

    let spreadsheet: Arc<Mutex<HashMap<CellIdentifier, CellValue>>> = Arc::new(Mutex::new(HashMap::new()));

    loop {
        info!("Just got message");
        match recv.read_message() {
            ReadMessageResult::Message(msg) => {
                // rsheet_lib already contains a FromStr<Command> (i.e. parse::<Command>)
                // implementation for parsing the get and set commands. This is just a
                // demonstration of how to use msg.parse::<Command>, you may want/have to
                // change this code.
                let mut sheet = spreadsheet.lock().unwrap();
                let mut set = false;
                let reply = match msg.parse::<Command>() {
                    Ok(command) => match command {
                        Command::Get { cell_identifier } => {
                          let id = format!("{}{}", column_number_to_name(cell_identifier.col), cell_identifier.row + 1);
                          let value = functionality::get::get(&sheet, cell_identifier);
                          Reply::Value(id, value)
                        },
                        Command::Set {
                            cell_identifier,
                            cell_expr,
                        } => {
                            set = true;
                            let id = format!("{}{}", column_number_to_name(cell_identifier.col), cell_identifier.row + 1);
                            let expression = CellExpr::new(&cell_expr);
                            let value = functionality::set::set(&mut sheet, cell_identifier, expression);
                            Reply::Value(id, value)
                        },
                    },
                    Err(e) => Reply::Error(e),
                };

                if !set {
                  match send.write_message(reply) {
                    WriteMessageResult::Ok => {
                        // Message successfully sent, continue.
                    }
                    WriteMessageResult::ConnectionClosed => {
                        // The connection was closed. This is not an error, but
                        // should terminate this connection.
                        break;
                    }
                    WriteMessageResult::Err(e) => {
                        // An unexpected error was encountered.
                        return Err(Box::new(e));
                    }
                  }
                }
            }
            ReadMessageResult::ConnectionClosed => {
                // The connection was closed. This is not an error, but
                // should terminate this connection.
                break;
            }
            ReadMessageResult::Err(e) => {
                // An unexpected error was encountered.
                return Err(Box::new(e));
            }
        }
    }
    Ok(())
}
