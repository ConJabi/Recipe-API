# Recipe API

A simple API for managing and retrieving recipes.

## Endpoints

### 1. `/api/recipes`
- **Description:** Returns a list of all available recipes.
- **Method:** GET

### 2. `/api/recipes/{id}`
- **Description:** Returns detailed info for recipe
- **Method:** GET

## Getting Started

To run, add .env file with database url like 	`DATABASE_URL=postgres://postgres:password@localhost:5432/postgres`
<br>
Then run cargo run 


For testing:
cargo test 

## To do list
- Create endpoints for the following: <br>
Add a new recipe  <br>
Delete a recipe  <br>
Search for a recipe 
- Add foreign key constraints to migration queries
- Fix testcontainer connection
- Populate DB in migration step for tests
- Add more tests
- Seperate DB and server setup from test function(s)

## Time block
| time (minutes)  | activity  |
|---|---|
|  80 | Initial setup, connecting the frameworks, familiarizing with frameworks  |  
|  15  | Database design, local database creation for testing |   
| 30 | adding parameters to actix, most time spend on debugging. Found that columns were set to charachters[] which is an array of textfields instead of a single texfield  
| 30 | Created two endpoints for recipes
   |85| Started on test environment, problems with connecting to testcontainer