mod domain;
mod infra;
mod use_cases;

use crate::infra::adapters::admin_routes::init as admin_init;
use crate::infra::adapters::public_routes::init as public_init;
use crate::infra::exporter::ics::IcsExporter;
use crate::use_cases::calendar_use_case::CalendarUseCase; // Ensure this is from the same crate
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use chrono::Utc;
use clap::{Parser, Subcommand};
use std::io::Write; // add this import

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
            let allowed_origin = std::env::var("ALLOWED_ORIGIN")
                .unwrap_or_else(|_| "http://localhost:3000".to_string());

            std::env::set_var("RUST_LOG", "actix_web=info");
            std::env::set_var("RUST_LOG_FORMAT", "json");
            env_logger::Builder::from_default_env()
                .format(|buf, record| {
                    writeln!(
                        buf,
                        "{{\"level\":\"{}\",\"time\":\"{}\",\"message\":\"{}\"}}",
                        record.level(),
                        Utc::now().to_rfc3339(),
                        record.args()
                    )
                })
                .init();

            HttpServer::new(move || {
                App::new()
                    .wrap(Logger::default())
                    .wrap(
                        Cors::default()
                            .allowed_origin(&allowed_origin)
                            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]),
                    )
                    .configure(public_init)
                    .configure(admin_init)
            })
            .bind("0.0.0.0:8080")?
            .run()
            .await
        }

        Commands::ExportIcal { .. } => {
            println!("Exporting calendars to ics files");
            let use_case = CalendarUseCase::new().expect("Failed to create CalendarUseCase");
            let exporter = IcsExporter::new();
            use_case
                .export(exporter)
                .expect("Failed to export calendars");
            println!("Exported calendars to ics files");
            Ok(())
        }
    }
}
