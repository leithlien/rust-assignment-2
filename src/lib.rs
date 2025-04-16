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
    // let (mut recv, mut send) = match manager.accept_new_connection() {
    //     Connection::NewConnection { reader, writer } => (reader, writer),
    //     Connection::NoMoreConnections => {
    //         // There are no more new connections to accept.
    //         return Ok(());
    //     }
    // };

    let spreadsheet: Arc<Mutex<HashMap<CellIdentifier, CellValue>>> = Arc::new(Mutex::new(HashMap::new()));

    std::thread::scope(|s| {
      loop {
        info!("Just got message");
        match manager.accept_new_connection() {
          Connection::NewConnection { reader, writer } => {
            let spreadsheet = Arc::clone(&spreadsheet);
            s.spawn(move || {
              let mut reader = reader;
              let mut writer = writer;
              loop {
                match reader.read_message() {
                  ReadMessageResult::Message(msg) => {
                    let res = match functionality::command::handle_command(msg, &mut reader, &mut writer, &spreadsheet) {
                      WriteMessageResult::Ok => (),
                      WriteMessageResult::ConnectionClosed => break,
                      WriteMessageResult::Err(e) => {
                        //return Err(Box::new(e))
                        eprintln!("{e}");
                        break;
                      },
                    };
                  }
                  ReadMessageResult::ConnectionClosed => {
                      // The connection was closed. This is not an error, but
                      // should terminate this connection.
                      break;
                  }
                  ReadMessageResult::Err(e) => {
                      // An unexpected error was encountered.
                      //return Err(Box::new(e));
                      eprintln!("{e}");
                      break;
                  }
                }
              }
            });
          },
          Connection::NoMoreConnections => break,
        }
      }
    });
    Ok(())
}
