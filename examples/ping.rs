use bb8_redis_cluster_async::RedisConnectionManager;
use redis::Cmd;

#[tokio::main]
async fn main() {
    let manager = RedisConnectionManager::new(vec![
        "redis://localhost:7000",
        "redis://localhost:7001",
        "redis://localhost:7002",
        "redis://localhost:7003",
        "redis://localhost:7004",
        "redis://localhost:7005",
    ])
    .unwrap();

    let pool = bb8::Pool::builder()
        .max_size(15)
        .build(manager)
        .await
        .unwrap();

    let mut handles = vec![];
    for _ in 0..20 {
        let pool = pool.clone();
        handles.push(tokio::spawn(async move {
            let mut conn = pool.get().await.unwrap();
            let reply: String = Cmd::new()
                .arg("PING")
                .query_async(&mut *conn)
                .await
                .unwrap();
            assert_eq!("PONG", reply);
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
