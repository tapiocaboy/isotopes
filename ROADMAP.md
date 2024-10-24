# ROADMAP.md

## Isotopes Backend Application Roadmap

This document outlines the roadmap for the development of the Isotopes backend REST application, which is responsible for managing the inventory of AI modules, raw modules, and machines. The app will be developed using Rust, with the existing project structure, and using `sqlx` for database interaction and migrations. Below are the main functionalities that will be added to the project.

### Features:

1. **Generate migration files for each new model**
    - Use `sqlx` to manage database migrations.
    - **Steps**:
      1. Add the new model structures (e.g., `ai_modules`, `raw_modules`, `machines`) to the Rust application.
      2. Use `sqlx-cli` to create migration files.
         - Example: 
           ```bash
           sqlx migrate add create_ai_modules
           sqlx migrate add create_raw_modules
           sqlx migrate add create_machines
           ```
      3. Update the generated migration files with the appropriate SQL for creating tables, relationships, and constraints.
      4. Apply the migrations to the database:
         - Example: 
           ```bash
           sqlx migrate run
           ```

2. **Subscribe to `ai_modules` updates**
    - Implement a service that subscribes to updates of `ai_modules` and handles these events.
    - **Steps**:
      1. Create a subscription endpoint for external services to push `ai_modules` updates.
      2. Add a handler to process incoming events and update the database accordingly.
      3. Ensure that subscription is resilient to failures and retries.

3. **Health Check API**
    - Implement a simple health check API to verify that the service is up and running.
    - **Steps**:
      1. Add a new route `/health`.
      2. Implement the handler to return HTTP status `200` and a message like `"Service is healthy"`.
      3. Add automated tests to ensure the health check endpoint is functional.
      4. Example response:
         ```json
         {
           "status": "ok"
         }
         ```

4. **Registration of `ai_modules`**
    - Implement CRUD operations for the `ai_modules` entity.
    - **Model Structure**:
      - `ai_id`
      - `business_id`
      - `module_id`
      - `created`
      - `updated`
      - `active`
    - **Steps**:
      1. Create a migration to define the `ai_modules` table.
      2. Implement the following REST endpoints:
         - `POST /ai_modules` – Create a new AI module.
         - `GET /ai_modules/{id}` – Get an AI module by ID.
         - `PUT /ai_modules/{id}` – Update an AI module.
         - `DELETE /ai_modules/{id}` – Delete an AI module.
      3. Validate the input and ensure database consistency.
      4. Add automated tests to cover all CRUD operations.

5. **Registration of `raw_modules`**
    - Implement CRUD operations for the `raw_modules` entity.
    - **Model Structure**:
      - `raw_module_id`
      - `module_name`
      - `module_code`
      - `hosted_link`
      - Relationship: One `raw_module` can have one or many `ai_modules`.
    - **Steps**:
      1. Create a migration for the `raw_modules` table.
      2. Implement the following REST endpoints:
         - `POST /raw_modules` – Create a new raw module.
         - `GET /raw_modules/{id}` – Get a raw module by ID.
         - `PUT /raw_modules/{id}` – Update a raw module.
         - `DELETE /raw_modules/{id}` – Delete a raw module.
      3. Ensure that relationships between `raw_modules` and `ai_modules` are respected.
      4. Add automated tests to cover all CRUD operations.

6. **Registration of `machines`**
    - Implement CRUD operations for the `machines` entity.
    - **Model Structure**:
      - `machine_id`
      - `machine_name`
      - `owner`
      - `created`
      - `updated`
      - `active`
      - `raw_module_id`
      - Relationship: One machine can have one or many `raw_modules`.
    - **Steps**:
      1. Create a migration for the `machines` table.
      2. Implement the following REST endpoints:
         - `POST /machines` – Create a new machine.
         - `GET /machines/{id}` – Get a machine by ID.
         - `PUT /machines/{id}` – Update a machine.
         - `DELETE /machines/{id}` – Delete a machine.
      3. Ensure relationships between `machines` and `raw_modules` are respected.
      4. Add automated tests for all CRUD operations.

7. **Usage Report of Isotopes**
    - Implement an API to generate usage reports for isotopes.
    - **Report Structure**:
      - `ai_id`
      - `machine_id`
      - `demand`
      - `total_up_hours`
    - **Steps**:
      1. Create an endpoint `/report/usage` to generate a report based on the above fields.
      2. The report should summarize the usage of `ai_modules` across different machines.
      3. Add filtering options based on `ai_id`, `machine_id`, or time range.
      4. Add automated tests to ensure report accuracy and performance.

---

## Future Improvements

- Implement authentication and authorization for all routes.
- Add rate-limiting and monitoring for APIs.
- Implement pagination for CRUD operations with large datasets.
- Add logging for better observability of errors and operations.
- Containerize the application with Docker for easier deployment.
