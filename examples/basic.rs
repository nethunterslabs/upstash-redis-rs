use upstash_redis_rs::{Command, Redis};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let url = std::env::var("UPSTASH_REDIS_REST_URL").expect("UPSTASH_REDIS_REST_URL is not set");
    let token =
        std::env::var("UPSTASH_REDIS_REST_TOKEN").expect("UPSTASH_REDIS_REST_TOKEN is not set");

    let redis = Redis::new(url, token).unwrap();

    let v = redis.hgetall("author")?.send().await?;

    println!("{:#?}", v);

    Ok(())
}
