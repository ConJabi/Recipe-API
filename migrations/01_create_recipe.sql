CREATE TABLE IF NOT EXISTS recipe
(
    id uuid NOT NULL,
    name VARCHAR(255) NOT NULL,
    time_required integer,
    meal_type VARCHAR(255),
    description TEXT NOT NULL,
    CONSTRAINT recipe_pkey PRIMARY KEY (id)
)