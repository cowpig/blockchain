extern crate redis;

use redis::Commands;

pub fn get_redisconn() -> redis::RedisResult<redis::Connection> {
    let client = match redis::Client::open("redis://127.0.0.1/") {
        Ok(client) => client,
        Err(err) => panic!("Failed to set up redis: {}", err),
    };
    match client.get_connection() {
        Ok(conn) => Ok(conn),
        Err(err) => panic!("Failed to connect to redis: {}", err),
    }
}

pub fn redis_pop(redis_conn: &redis::Connection, key: &String) -> redis::RedisResult<String> {
    let val: Option<String> = redis_conn.rpop(key)?;  // rpop fail returns RedisResult(Err(reason))
    match val {
        Some(val) => Ok(val),
        None => Ok("".to_string()),
    }
}

pub fn redis_push(redis_conn: &redis::Connection, key: &String, val: String) -> redis::RedisResult<()> {
    let _ : () = try!(redis_conn.rpush(key, val));
    Ok(())
}
