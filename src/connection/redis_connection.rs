use redis::{AsyncCommands, Client, Commands, RedisError, aio::{MultiplexedConnection, PubSub}};
use std::env;

pub fn get_redis_url() -> String {
    dotenvy::dotenv().ok();

    env::var("REDIS_URL").unwrap_or_else(|_| {
        let host = env::var("REDIS_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string());
        let password = env::var("REDIS_PASSWORD").ok();

        if let Some(pw) = password {
            format!("redis://:{}@{}:{}/", pw, host, port)
        } else {
            format!("redis://{}:{}/", host, port)
        }
    })
}

pub fn redis_connection() -> Result<(), RedisError> {
    let redis_url = get_redis_url();

    let client = Client::open(redis_url.as_str())?;
    let mut conn = client.get_connection()?;

    let _: () = conn.set("name", "John")?;
    let value: String = conn.get("name")?;

    println!("Value: {}", value);

    Ok(())
}

async fn get_redis_connection() -> Result<MultiplexedConnection, RedisError> {
    let redis_url = get_redis_url();

    let client = Client::open(redis_url.as_str())?;
    let conn = client.get_multiplexed_async_connection().await?;

    Ok(conn)
}

async fn redis_subscribe_pubsub() -> Result<PubSub, RedisError> {
    let redis_url = get_redis_url();
    let client = Client::open(redis_url.as_str())?;
    let mut pubsub = client.get_async_pubsub().await?;

    Ok(pubsub)
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, num::NonZero};

    use redis::{
        Value,
        AsyncCommands,
        geo::{RadiusOptions, Unit},
        streams::{StreamReadOptions, StreamReadReply},
    };

    use futures::StreamExt;

    use super::*;

    #[test]
    fn test_redis_connection() {
        redis_connection().expect("failed to connect to redis");
    }

    #[tokio::test]
    async fn test_redis_connection_async() -> Result<(), RedisError> {
        let mut conn = get_redis_connection().await?;
        let _: () = conn.set("name", "John").await?;
        let value: String = conn.get("name").await?;

        println!("Value: {}", value);

        Ok(())
    }

    #[tokio::test]
    async fn test_string() -> Result<(), RedisError> {
        let mut conn = get_redis_connection().await?;
        let _: () = conn.set_ex("greeting", "Hello, World!", 2).await?; //set with expiration of 2 seconds
        let value: String = conn.get("greeting").await?;

        println!("value {}", value);
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await; //wait for 5 seconds

        let value: Result<String, RedisError> = conn.get("greeting").await;
        assert_eq!(true, value.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_list() -> Result<(), RedisError> {
        let mut conn = get_redis_connection().await?;

        let _: () = conn.del("mylist").await?;
        let _: () = conn.rpush("mylist", "item1").await?;
        let _: () = conn.rpush("mylist", "item2").await?;

        let len: i32 = conn.llen("mylist").await?;
        assert_eq!(2, len);

        let items: Vec<String> = conn.lrange("mylist", 0, -1).await?;
        assert_eq!(vec!["item1", "item2"], items);

        let item: Vec<String> = conn.lpop("mylist", NonZero::new(1)).await?;
        assert_eq!(vec!["item1"], item);
        Ok(())
    }

    #[tokio::test]
    async fn test_set() -> Result<(), RedisError> {
        let mut conn = get_redis_connection().await?;

        let _: () = conn.del("mySet").await?;
        let _: () = conn.sadd("mySet", "item1").await?;
        let _: () = conn.sadd("mySet", "item1").await?;
        let _: () = conn.sadd("mySet", "item1").await?;
        let _: () = conn.sadd("mySet", "item2").await?;
        let _: () = conn.sadd("mySet", "item2").await?;
        let _: () = conn.sadd("mySet", "item2").await?;
        let _: () = conn.sadd("mySet", "item3").await?;

        let len: i32 = conn.scard("mySet").await?;
        assert_eq!(3, len);

        let items: Vec<String> = conn.smembers("mySet").await?;
        assert_eq!(vec!["item1", "item2", "item3"], items);

        Ok(())
    }

    #[tokio::test]
    async fn test_set_sort() -> Result<(), RedisError> {
        let mut conn = get_redis_connection().await?;

        let _: () = conn.del("mySetSort").await?;
        let _: () = conn.zadd("mySetSort", "item1", 100).await?;
        let _: () = conn.zadd("mySetSort", "item2", 85).await?;
        let _: () = conn.zadd("mySetSort", "item3", 95).await?;

        let len: i32 = conn.zcard("mySetSort").await?;
        assert_eq!(3, len);

        let items: Vec<String> = conn.zrange("mySetSort", 0, -1).await?;
        assert_eq!(vec!["item2", "item3", "item1"], items);

        Ok(())
    }

    #[tokio::test]
    async fn test_hash_set() -> Result<(), RedisError> {
        let mut conn = get_redis_connection().await?;

        let _: () = conn.del("mySetHash:1").await?;
        let _: () = conn.hset("mySetHash:1", "id", "1").await?;
        let _: () = conn.hset("mySetHash:1", "name", "HD").await?;
        let _: () = conn.hset("mySetHash:1", "email", "hd@mail.co").await?;

        let user: HashMap<String, String> = conn.hgetall("mySetHash:1").await?;
        assert_eq!(user.get("id").unwrap(), "1");
        assert_eq!(user.get("name").unwrap(), "HD");
        assert_eq!(user.get("email").unwrap(), "hd@mail.co");

        Ok(())
    }

    #[tokio::test]
    async fn test_geo_point() -> Result<(), RedisError> {
        let mut conn = get_redis_connection().await?;

        let _: () = conn.del("sellers").await?;
        let _: () = conn
            .geo_add("sellers", (106.822702, -6.177590, "Toko A"))
            .await?;
        let _: () = conn
            .geo_add("sellers", (106.820889, -6.174964, "Toko B"))
            .await?;

        let distance: f64 = conn
            .geo_dist("sellers", "Toko A", "Toko B", Unit::Kilometers)
            .await?;
        println!("Distance: {} kilometers", distance);
        assert_eq!(0.3543, distance);

        let result: Vec<String> = conn
            .geo_radius(
                "sellers",
                106.821825,
                -6.175105,
                5.0,
                Unit::Kilometers,
                RadiusOptions::default(),
            )
            .await?;
        println!("Distance: {:?}", result);
        assert_eq!(vec!["Toko A", "Toko B"], result);

        Ok(())
    }

    #[tokio::test]
    async fn test_hyper_log_log() -> Result<(), RedisError> {
        let mut conn = get_redis_connection().await?;

        let _: () = conn.del("visitors").await?;
        let _: () = conn.pfadd("visitors", ("Dian", "Rahmat", "Hadi")).await?;
        let _: () = conn.pfadd("visitors", ("Rahmat", "Jupri", "Budi")).await?;
        let _: () = conn.pfadd("visitors", ("Budi", "joko", "Abul")).await?;

        let total: i32 = conn.pfcount("visitors").await?;
        println!("Total unique visitors: {}", total);
        assert_eq!(7, total);

        Ok(())
    }

    #[tokio::test]
    async fn test_pipeline() -> Result<(), RedisError> {
        let mut conn = get_redis_connection().await?;

        redis::pipe()
            .set_ex("name", "hadi", 2)
            .set_ex("address", "Bekasi", 2)
            .exec_async(&mut conn)
            .await?;

        let name: String = conn.get("name").await?;
        println!("Name: {}", name);
        assert_eq!("hadi", name);

        let address: String = conn.get("address").await?;
        println!("Address: {}", address);
        assert_eq!("Bekasi", address);

        Ok(())
    }

    #[tokio::test]
    async fn test_pipeline_transaction() -> Result<(), RedisError> {
        let mut conn = get_redis_connection().await?;

        redis::pipe()
            .atomic()
            .set_ex("name", "hadi", 2)
            .set_ex("address", "Bekasi", 2)
            .exec_async(&mut conn)
            .await?;

        let name: String = conn.get("name").await?;
        println!("Name: {}", name);
        assert_eq!("hadi", name);

        let address: String = conn.get("address").await?;
        println!("Address: {}", address);
        assert_eq!("Bekasi", address);

        Ok(())
    }

    #[tokio::test]
    async fn test_stream() -> Result<(), RedisError> {
        let mut conn = get_redis_connection().await?;

        for i in 1..=10 {
            let mut map: HashMap<&str, String> = HashMap::new();
            map.insert("sensor_id", "sensor_1".to_string());
            map.insert("temperature", format!("{} derajat", i));

            let _: () = conn.xadd_map("sensor_data", "*", &map).await?;
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_create_consumer() -> Result<(), RedisError> {
        let mut conn = get_redis_connection().await?;

        let _: () = conn
            .xgroup_create("sensor_data", "sensor_group", "0")
            .await?;
        let _: () = conn
            .xgroup_createconsumer("sensor_data", "sensor_group", "consumer_1")
            .await?;
        let _: () = conn
            .xgroup_createconsumer("sensor_data", "sensor_group", "consumer_2")
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_stream() -> Result<(), RedisError> {
        let mut conn = get_redis_connection().await?;

        let setting = StreamReadOptions::default()
            .group("sensor_group", "consumer_1")
            .count(5)
            .block(3000);

        let result: StreamReadReply = conn
            .xread_options(&["sensor_data"], &[">"], &setting)
            .await?;

        for key in result.keys {
            for item in key.ids {
                println!("ID: {}", item.id);

                let map: HashMap<String, Value> = item.map;
                let name: String = match map.get("sensor_id").unwrap() {
                    Value::BulkString(value) => String::from_utf8(value.to_vec()).unwrap(),
                    _ => "".to_string(),
                };

                let address: String = match map.get("temperature").unwrap() {
                    Value::BulkString(value) => String::from_utf8(value.to_vec()).unwrap(),
                    _ => "".to_string(),
                };
                println!("{:?}", name);
                println!("{:?}", address);
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_pubsub_subscribe() -> Result<(), RedisError> {
        let mut pubsub = redis_subscribe_pubsub().await?;

        let _: () = pubsub.subscribe("sensor_data").await?;
        let mut pubsub_stream = pubsub.on_message();

        let message: String = pubsub_stream.next().await.unwrap().get_payload()?;
        println!("Received message: {:?}", message);
        Ok(())
    }

    #[tokio::test]
    async fn test_send_pubsub() -> Result<(), RedisError> {
        let mut conn = get_redis_connection().await?;
        let _: i32 = conn.publish("sensor_data", "Hello from Redis Pub/Sub!").await?;

        Ok(())
    }
}
