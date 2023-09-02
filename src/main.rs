use clap::{Args, Parser, Subcommand};
use serde::Deserialize;
use sql2xlsx::Query;
use std::error::Error;
use std::path::PathBuf;
use std::{env, fs};

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(long)]
    db: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    File(FromFile),
    Sql(FromSql),
}

#[derive(Args, Debug)]
struct FromFile {
    #[arg(long)]
    name: String,
}

impl FromFile {
    fn into_from_sql(self) -> Result<FromSql, Box<dyn Error>> {
        let file_content = fs::read_to_string(&self.name).map_err(|err| {
            eprintln!("Read file {} err: {:?}", &self.name, err);
            err
        })?;

        toml::from_str(&file_content).map_err(|err| {
            eprintln!("Deserialize file {} err: {:?}", &self.name, err);
            err.into()
        })
    }
}

#[derive(Args, Debug, Deserialize)]
struct FromSql {
    #[arg(long)]
    header: String,
    #[arg(long)]
    sql: String,
    #[arg(long)]
    out: PathBuf,
}

impl FromSql {
    fn into_query(self, db_url: String) -> Query {
        Query::new(db_url.to_string(), self.sql, self.header, self.out)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    if dotenvy::dotenv().is_err() {
        println!("NO .env file found");
    }

    let cli = Cli::parse();

    let db_url = match cli.db.or_else(|| env::var("DATABASE_URL").ok()) {
        Some(db) => db,
        None => {
            eprintln!("NO DB_URL");
            return Ok(());
        }
    };

    let from_sql = match cli.command {
        Commands::File(from_file) => from_file.into_from_sql()?,
        Commands::Sql(from_sql) => from_sql,
    };

    let query = from_sql.into_query(db_url);
    query.execute()
}
