# Postal Code Service

A high-performance REST API service written in Rust that provides city and state information for US postal codes (ZIP codes). The service loads postal code data from a CSV file into memory and serves requests using a thread pool for optimal performance.

## Features

- Fast in-memory ZIP code lookups
- RESTful GET endpoint
- Thread pool for concurrent request handling
- Response caching headers
- Optimized for high throughput
- Worker count based on CPU cores

## Prerequisites

- Rust (latest stable version)
- CSV file with postal code data (`ZIP_Locale_Detail.csv`)

## Installation

1. Clone the repository: 

```bash
git clone <repository-url>
cd postal-service
```

2. Build the project:

```bash
cargo build
```

3. Run the project:

```bash
cargo run --release
```

4. Test the project:

```bash
cargo test
```

5. Run the project:

```bash
cargo run --release
```

The service will:
1. Load the ZIP code database into memory
2. Print the number of loaded ZIP codes
3. Start the HTTP server on `http://127.0.0.1:8080`

## API Usage

### Get Location by ZIP Code

```
GET /location/{postal_code}
```

Response

```
{
"city": "ADJUNTAS",
"state": "PR"
}
```

## Performance

The service is optimized for performance with:
- Pre-allocated HashMap for ZIP code data
- Thread pool for request handling
- Response caching (24-hour cache headers)
- Keep-alive connections
- Worker optimization based on CPU cores

## CSV File Format

The service expects a CSV file (`ZIP_Locale_Detail.csv`) with the following columns:
- Column 4: DELIVERY ZIPCODE
- Column 7: PHYSICAL CITY
- Column 8: PHYSICAL STATE

You can download the XLS of this file from the USPS website: https://postalpro.usps.com/ZIP_Locale_Detail and then convert it to CSV using Excel.
