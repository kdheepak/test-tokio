time ./target/release/test-tokio normal spawn print >/dev/null

time ./target/release/test-tokio normal spawn sleep >/dev/null

time ./target/release/test-tokio normal spawn_blocking print >/dev/null

time ./target/release/test-tokio normal spawn_blocking sleep >/dev/null

# time ./target/release/test-tokio reversed spawn print >/dev/null

time ./target/release/test-tokio reversed spawn sleep >/dev/null

# time ./target/release/test-tokio reversed spawn_blocking print >/dev/null

# time ./target/release/test-tokio reversed spawn_blocking sleep >/dev/null
