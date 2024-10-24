# Isotopes Backend Application

The Isotopes backend application is a RESTful service developed in Rust, designed to manage the inventory of AI modules, raw modules, and machines. It uses `sqlx` for database interactions and migrations, and is structured to support efficient CRUD operations and data management.

## Features

- **AI Modules Management**: 
  - Create, read, update, and delete AI modules.
  - Each AI module has a unique ID, business ID, module ID, and status (active/inactive).
  - Timestamps for creation and updates are automatically managed.
  - Indexes on `business_id` and `module_id` for efficient querying.
  
  ```
  sql:migrations/20241024051838_create_ai_modules.sql
  ```

- **Raw Modules Management**:
  - Manage raw modules with unique IDs, names, codes, and hosted links.
  - Indexed by module name for quick access.
  
  ```sql:migrations/20241024051839_create_raw_modules.sql
  ```

- **Machines Management**:
  - Handle machine records with unique IDs, names, owners, and associated raw modules.
  - Foreign key relationship with raw modules ensures data integrity.
  - Indexed by owner and raw module ID.
  
  ```sql:migrations/20241024051840_create_machines.sql
  ```

- **Subscriptions**:
  - Manage subscriptions with unique UUIDs, emails, names, and subscription timestamps.
  
  ```sql:migrations/20241013074559_create_subscriptions_table.sql

  ```

## Getting Started

### Prerequisites

- Rust and Cargo installed
- PostgreSQL database
- `sqlx-cli` for managing migrations

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/isotopes.git
   cd isotopes
   ```

2. Set up the database:
   - Ensure PostgreSQL is running.
   - Create a database and update the connection string in your `.env` file.

3. Run migrations:

   ```bash
   sqlx migrate run
   ```

4. Build and run the application:

   ```bash
   cargo build
   cargo run
   ```

### Testing

Run tests using:

```bash
cargo test
```


## Continuous Integration

The project uses GitHub Actions for CI/CD. The workflow includes building the project, running tests, and incrementing the version on successful builds.

## License

This project is licensed under the MIT License.

## Contact


