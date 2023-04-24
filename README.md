# Next-Rocket-Diesel-Postgres CRUD

A basic CRUD application (todo list) using:

- Next.js for the frontend
- Rocket for the backend
- Diesel as the ORM
- PostgreSQL as the database

## Routes

- `GET /api/tasks` - Get all tasks
- `POST /api/tasks` - Create a new task
- `PATCH /api/tasks?id=<i32>&state=<bool>` - Update a task's status
- `DELETE /api/tasks?id=<i32>` - Delete a task