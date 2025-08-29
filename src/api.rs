use actix_web::{web, Responder, HttpResponse, get};
use sqlx::{PgPool, FromRow};
use serde::{Serialize, Deserialize};

use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, FromRow)]
struct RecipeOverview {
    id: Uuid,
    name: String,
    meal_type: Option<String>,
    time_required: Option<i32>,
}

/// Represents a single ingredient within a recipe.
#[derive(Serialize, Deserialize, Debug)]
struct Ingredient {
    name: String,
    amount: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RecipeDetails {
    id: Uuid,
    name: String,
    description: String,
    meal_type: Option<String>,
    time_required: Option<i32>,
    ingredients: Vec<Ingredient>,
}


#[get("/recipes")]
async fn get_recipes_overview(pool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let recipes = sqlx::query_as!(
        RecipeOverview,
        "SELECT id, name, meal_type, time_required FROM recipe ORDER BY name"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    eprintln!("Fetched {} recipes", recipes.len());
    Ok(HttpResponse::Ok().json(recipes))
}

#[get("/recipes/{id}")]
async fn get_recipe_details(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let recipe_id = path.into_inner();

    // Fetch the main recipe details.
    let recipe_row = sqlx::query!(
        "SELECT id, name, description, meal_type, time_required FROM recipe WHERE id = $1",
        recipe_id
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Error fetching recipe: {:?}", e);
        actix_web::error::ErrorNotFound("Recipe not found or database error")
    })?;

    // Fetch the ingredients for the recipe.
    let ingredients = sqlx::query!(
        "SELECT i.name, ri.amount FROM ingredient i
         JOIN recipe_ingredient ri ON i.id = ri.ingredient_id
         WHERE ri.recipe_id = $1",
        recipe_id
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let recipe_details = RecipeDetails {
        id: recipe_row.id,
        name: recipe_row.name,
        description: recipe_row.description,
        meal_type: recipe_row.meal_type,
        time_required: recipe_row.time_required,
        ingredients: ingredients
            .into_iter()
            .map(|row| Ingredient {
                name: row.name,
                amount: row.amount,
            })
            .collect(),
    };

    Ok(HttpResponse::Ok().json(recipe_details))
}


// Function to configure all API routes
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_recipes_overview)
            .service(get_recipe_details)

    );
}