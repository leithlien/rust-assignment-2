pub fn handle_command(
  mut recv: Reader, 
  mut send: Writer, 
  spreadsheet: HashMap<CellIdentifier, CellValue>
) -> WriteMessageResult {
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