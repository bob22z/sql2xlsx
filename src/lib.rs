use postgres::{Client, NoTls, SimpleQueryMessage};
use rust_xlsxwriter::{ColNum, RowNum, Workbook, Worksheet};
use std::error::Error;
use std::path::PathBuf;

pub struct Query {
    db: String,
    sql: String,
    header: String,
    file_name: PathBuf,
}

impl Query {
    pub fn new(db: String, sql: String, header: String, file_name: PathBuf) -> Query {
        Query {
            db,
            sql,
            header,
            file_name,
        }
    }

    pub fn execute(self) -> Result<(), Box<dyn Error>> {
        let mut client = Client::connect(&self.db, NoTls)?;

        let rows = client.simple_query(&self.sql)?;

        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        write_header(&self.header, worksheet)?;

        write_body(rows, worksheet)?;

        workbook.save(&self.file_name)?;

        println!("Saved file to: {}", self.file_name.display());

        Ok(())
    }
}

fn write_header(header: &str, worksheet: &mut Worksheet) -> Result<(), Box<dyn Error>> {
    for (col, cell) in header.split(',').enumerate() {
        worksheet.write_string(0, col as ColNum, cell.trim())?;
    }
    Ok(())
}

fn write_body(
    rows: Vec<SimpleQueryMessage>,
    worksheet: &mut Worksheet,
) -> Result<(), Box<dyn Error>> {
    for (row_index, row) in rows.into_iter().enumerate() {
        if let SimpleQueryMessage::Row(simple_row) = row {
            for col in 0..simple_row.len() {
                worksheet.write_string(
                    (row_index + 1) as RowNum,
                    col as ColNum,
                    simple_row.get(col).unwrap_or_default(),
                )?;
            }
        }
    }
    Ok(())
}
