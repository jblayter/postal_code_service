use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use tokio::runtime::Handle;
use std::time::Duration;

// Create a thread pool for handling requests
lazy_static! {
    static ref THREAD_POOL: rayon::ThreadPool = rayon::ThreadPoolBuilder::new()
        .num_threads(20)
        .build()
        .unwrap();

    static ref ZIPCODE_MAP: HashMap<String, Location> = {
        println!("Loading ZIP code database...");
        let mut map = HashMap::with_capacity(50000); // Pre-allocate space
        let file = File::open("ZIP_Locale_Detail.csv").expect("Failed to open CSV file");
        let reader = BufReader::new(file);
        let mut csv_reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(reader);

        for result in csv_reader.records() {
            if let Ok(record) = result {
                if let (Some(zipcode), Some(city), Some(state)) = (
                    record.get(4),
                    record.get(7),
                    record.get(8),
                ) {
                    map.insert(
                        zipcode.to_string(),
                        Location {
                            city: city.to_string(),
                            state: state.to_string(),
                        },
                    );
                }
            }
        }
        println!("Loaded {} ZIP codes", map.len());
        map
    };
}

#[derive(Serialize, Deserialize, Clone)]
struct Location {
    city: String,
    state: String,
}

fn lookup_postal_code(postal_code: &str) -> Option<Location> {
    ZIPCODE_MAP.get(postal_code).cloned()
}

async fn get_location(path: web::Path<String>) -> impl Responder {
    let postal_code = path.into_inner();
    let handle = Handle::current();

    // Use the thread pool instead of spawning new threads
    let result = THREAD_POOL.install(|| {
        handle.block_on(async {
            lookup_postal_code(&postal_code)
        })
    });

    match result {
        Some(location) => {
            // Add caching headers for successful responses
            HttpResponse::Ok()
                .insert_header(("Cache-Control", "public, max-age=86400")) // Cache for 24 hours
                .insert_header(("ETag", format!("\"{}\"", postal_code)))
                .json(location)
        }
        None => HttpResponse::NotFound().json("Postal code not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Force initialization of static resources
    let _pool = &*THREAD_POOL;
    
    println!("Server starting at http://127.0.0.1:8080");
    
    HttpServer::new(move || {
        App::new()
            // Add request logging middleware
            .wrap(actix_web::middleware::Logger::default())
            .service(
                web::resource("/location/{postal_code}")
                    .route(web::get().to(get_location))
            )
    })
    .workers(num_cpus::get()) // Optimize number of workers based on CPU cores
    .keep_alive(Duration::from_secs(30))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
