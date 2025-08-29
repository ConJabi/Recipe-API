CREATE TABLE IF NOT EXISTS recipe
(
    id uuid NOT NULL,
    name VARCHAR(255) COLLATE pg_catalog."default" NOT NULL,
    time_required integer,
    meal_type VARCHAR(255) COLLATE pg_catalog."default",
    description TEXT COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT recipe_pkey PRIMARY KEY (id)
)