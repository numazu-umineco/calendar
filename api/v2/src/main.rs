mod domain;
mod infra;
mod use_cases;

use crate::infra::adapters::admin_routes::init as admin_init;
use crate::infra::adapters::public_routes::init as public_init;
use crate::infra::exporter::ics::IcsExporter;
use crate::use_cases::calendar_use_case::CalendarUseCase; // Ensure this is from the same crate
use actix_web::{App, HttpServer};
use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Run {},
    ExportIcal {},
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Run { .. } => {
            HttpServer::new(|| App::new().configure(public_init).configure(admin_init))
                .bind("127.0.0.1:8080")?
                .run()
                .await
        }

        Commands::ExportIcal { .. } => {
            let use_case = CalendarUseCase::new().expect("Failed to create CalendarUseCase");
            let exporter = IcsExporter::new();
            use_case
                .export(exporter)
                .expect("Failed to export calendars");
            Ok(())
        }
    }
}
