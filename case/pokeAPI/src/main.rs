use serde::{Deserialize, Serialize};
use serde_json;
use mysql::*;
use mysql::prelude::*;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::FromRow;

#[derive(Deserialize, Debug, Serialize, FromRow)]
pub struct Pokemon {
    name: String,
    height: i32,
    weight: i32,
    pokedex: i8,
}

fn main() {
    let name = String::from("charizard");
    // let fetch = fetch_pokemon(name.clone());
    // add_pokemon_to_db(fetch);
    get_pokemon_from_db(name);
}


fn fetch_pokemon(name: String) -> Pokemon{
    let url = format!("testurl/{}", name);
    let data = r#"
    {
        "name": "Charizard",
        "height": 10,
        "weight": 10,
        "pokedex": 3
    }
    "#;
    let json: Pokemon = serde_json::from_str(data).unwrap();
    // ask_api(url);
    // return json
    json
}

fn add_pokemon_to_db(json: Pokemon) -> Result<()>{
    let database_url = "mysql://root:@127.0.0.1:3306/pokeapi";
    let pool = Pool::new(database_url)?;
    let mut conn = pool.get_conn()?;

    conn.exec_drop(
        r"INSERT INTO pokemon (name, height, weight, pokedex) VALUES (:name, :height, :weight, :pokedex)",
        params! {
            "name" => &json.name,
            "height" => &json.height,
            "weight" => &json.weight,
            "pokedex" => &json.pokedex,
        },
    )?;

    Ok(())
}

#[tokio::main]
async fn get_pokemon_from_db(name: String) -> Result<(), sqlx::Error> {
    let database_url = "mysql://root:@127.0.0.1:3306/pokeapi";

    let pool = MySqlPoolOptions::new()
        .connect(database_url)
        .await?;

    let result = sqlx::query_as::<_, Pokemon>(
        "SELECT name, height, weight, pokedex FROM pokemon WHERE name = ?"
    )
    .bind(&name)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(pokemon) => println!("Found pokemon: {} with height {} and weight {} and pokedex {}", pokemon.name, pokemon.height, pokemon.weight, pokemon.pokedex),
        Err(_) => println!("No pokemon named {} found", name)
    };
   
    Ok(())

}


