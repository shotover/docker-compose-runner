use docker_compose_runner::{DockerCompose, Image};
use redis::Commands;

#[test]
fn test() {
    // loop multiple times to test cleanup
    for _ in 0..3 {
        let _redis = DockerCompose::new(get_image_waiters(), |_| {}, "tests/docker-compose.yaml");
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_connection().unwrap();
        let _: () = con.set("my_key", 42).unwrap();
        let result: i32 = con.get("my_key").unwrap();
        assert_eq!(result, 42);
    }
}

pub fn get_image_waiters() -> &'static [Image] {
    &[Image {
        name: "bitnami/redis:6.2.13-debian-11-r73",
        log_regex_to_wait_for: r"Ready to accept connections",
        timeout: 120,
    }]
}
