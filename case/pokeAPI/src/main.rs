use serde::{Deserialize, Serialize};
use serde_json;
use mysql::*;
use mysql::prelude::*;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::FromRow;
use std::io::{self};
use serde_json::Value;
use reqwest;
use std::error::Error;


#[derive(Deserialize, Debug, Serialize, FromRow)]
pub struct Pokemon {
    name: String,
    height: i64,
    weight: i64,
    pokedex: i64,
}

#[derive(Debug, FromRow)]
pub struct Type {
    r#type: String
}

fn main() {
    let name = get_pokemon_name();
    let Ok(_) = get_pokemon_from_db(name.clone()) else {
        // get_pokemon_from_ai(&name);
        let fetch: Value = match fetch_pokemon(name.clone()) {
        Ok(val) => val,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
        };
        let (pokemon, r#type) = get_pokemon_information(fetch);
        let _ = add_pokemon_to_db(pokemon, r#type);
        let _ = get_pokemon_from_db(name);
        return 
    };
}

fn fetch_pokemon(name: String) -> Result<Value, String>{
    let url  = format!("https://pokeapi.co/api/v2/pokemon/{}", name);
    let resp = reqwest::blocking::get(url).map_err(|_| "Network request failed".to_string())?;
    if !resp.status().is_success() {
        return Err(format!("Pokemon {} not found (status {})", name, resp.status()))
    }

    let data: Value = resp
        .json()
        .map_err(|_| "Failed to parse JSON".to_string())?;

    Ok(data)
}

fn add_pokemon_to_db(json: Pokemon, r#type: Vec<String>) -> Result<(), Box<dyn Error>>{

    let settings = config::Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()
        .map_err(|e| format!("Failed to load config: {}", e))?;

    let database_url: String = settings
        .get("database.url")
        .map_err(|e| format!("Wrong or missing url: {}", e))?;
    
    let pool = Pool::new(database_url.as_str())
        .map_err(|e| format!("Failed to create DB pool: {}", e))?; 
    
    let mut conn = pool
        .get_conn()
        .map_err(|e| format!("Failed to get DB connection: {}", e))?;

    let mut tx = conn.start_transaction(TxOpts::default())?;

    tx.exec_drop(
        r"INSERT INTO pokemon (name, height, weight, pokedex) VALUES (:name, :height, :weight, :pokedex)",
        params! {
            "name" => &json.name,
            "height" => &json.height,
            "weight" => &json.weight,
            "pokedex" => &json.pokedex,
        },
    )?;

    let pokemon_id = tx.last_insert_id();

    for type_name in &r#type {
        let maybe_type_id:Option<u64> = tx.exec_first(
            "SELECT id FROM types WHERE type = :type",
            params! {
                "type" => type_name
            },)?;
        let type_id = if let Some(id) = maybe_type_id {
            id
        } else {
            tx.exec_drop(
                "INSERT INTO types (type) VALUES (:type)",
                params! {
                    "type" => type_name
                },)?;
                tx.last_insert_id().unwrap()
        };
        tx.exec_drop(
            "INSERT INTO pokemon_types (pokemon_id, type_id) VALUES (:pokemon_id, :type_id)",
            params! {
                "pokemon_id" => pokemon_id,
                "type_id" => type_id,
            },)?;
    }
    tx.commit()?;
    Ok(())
}

#[tokio::main]
async fn get_pokemon_from_db(name: String) -> Result<(), Box<dyn Error>> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()
        .map_err(|e| format!("Failed to load config: {}", e))?;

    let database_url: String = settings
        .get("database.url")
        .map_err(|e| format!("Wrong or missing url: {}", e))?;

    let pool = MySqlPoolOptions::new()
        .connect(&database_url)
        .await
        .map_err(|e| format!("Failed to connect to DB: {}", e))?;

    let pokemon = sqlx::query_as::<_, Pokemon>(
        "SELECT id, name, height, weight, pokedex FROM pokemon WHERE name = ?"
    )
    .bind(&name)
    .fetch_one(&pool)
    .await
    .map_err(|e| format!("Query failed: {}", e))?;

    let types  = sqlx::query_as::<_, Type>(
        r#"
        SELECT t.type 
        FROM types t
        JOIN pokemon_types pt ON pt.type_id = t.id
        JOIN pokemon p ON p.id = pt.pokemon_id
        WHERE p.name = ?
        ORDER BY t.id
        "#
    )
    .bind(&name)
    .fetch_all(&pool)
    .await
    .map_err(|e| format!("Query failed: {}", e))?;

    let poke_types = types.iter().map(|t| &t.r#type).collect::<Vec<_>>();

    println!("The pokemon is {}", pokemon.name);
    println!("{}'s height is {}", pokemon.name, pokemon.height);
    println!("{}'s weight is {}", pokemon.name, pokemon.weight);
    println!("{}'s pokedex number is {}", pokemon.name, pokemon.pokedex);
    for (i, poke_type) in poke_types.iter().enumerate() {
        if i == 0 {
            print!("{}'s type is {} ", pokemon.name, poke_type)
        }
        if i ==1 {
            print!("and {}", poke_type)
        }
    }
   
    Ok(())
}


// fn read_file() -> Result<String> {
//     let mut file = File::open("test.json")?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(contents)
// }

fn get_pokemon_information(string_data: Value) -> (Pokemon, Vec<String>) {
    let mut pokemon_name: Option<String> = None;
    let mut pokemon_dex: Option<i64> = None;
    let mut pokemon_type: Vec<String> = vec![];
    if let Some(forms) = string_data["forms"].as_array() {
        for form in forms {
            let name = form["name"].as_str().unwrap();
            pokemon_name = Some(name.to_string());
        }
    }
    if let Some(types) = string_data["types"].as_array() {
        for r#type in types {
            let poke_type = r#type["type"]["name"].as_str().unwrap();
            pokemon_type.push(poke_type.to_string());
        }
    }
    let pokemon_height= string_data["height"].as_i64().unwrap();
    let pokemon_weight= string_data["weight"].as_i64().unwrap();
    if let Some(game_indices) = string_data["game_indices"].as_array() {
        if let Some(entry) = game_indices.iter().find(|e|e["version"]["name"] == "gold") {
            let pokedex = entry["game_index"].as_i64().unwrap();
            pokemon_dex = Some(pokedex);
        }
    }
    let pokemon = Pokemon {
        name: pokemon_name.unwrap(),
        height: pokemon_height, 
        weight: pokemon_weight,
        pokedex: pokemon_dex.unwrap()
    };

    (pokemon, pokemon_type)
}

fn get_pokemon_name() -> String {
    println!("Please input your pokemon.");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.pop();
    input.pop();
    String::from(input)
}

// fn get_pokemon_from_ai(name: &str){

// }