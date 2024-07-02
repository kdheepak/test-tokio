# test-tokio

## Results

### Spawn fib and print to stdout:

```rust
    let r = tokio::task::spawn(async { fib(NUMBER) });

    while !r.is_finished() {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(1000)
            .map(char::from)
            .collect();
        println!("{}", s);
    }

    let r = r.await?;
```

```
78.52s user 2.01s system 198% cpu 40.498 total
```

### Spawn fib and sleep:

```rust
    let r = tokio::task::spawn(async { fib(NUMBER) });

    while !r.is_finished() {
        tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
    }

    let r = r.await?;
```

```
39.34s user 0.03s system 99% cpu 39.521 total
```

### Spawn blocking fib and print to stdout:

```rust
    let r = tokio::task::spawn_blocking(|| fib(NUMBER));

    while !r.is_finished() {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(1000)
            .map(char::from)
            .collect();
        println!("{}", s);
    }

    let r = r.await?;
```

```
78.41s user 2.04s system 199% cpu 40.325 total
```

### Spawn blocking fib and sleep:

```rust
    let r = tokio::task::spawn_blocking(|| fib(NUMBER));

    while !r.is_finished() {
        tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
    }

    let r = r.await?;
    println!("{r}");
```

```
39.18s user 0.03s system 99% cpu 39.347 total
```

---

### Spawn sleep loop and call fib

```rust
    let r = tokio::task::spawn(async {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
        }
    });

    fib(NUMBER);

    r.abort();
```

```
39.16s user 0.02s system 99% cpu 39.299 total
```

### Spawn print loop and call fib

```rust
    let r = tokio::task::spawn(async {
        loop {
            let s: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(1000)
                .map(char::from)
                .collect();
            println!("{}", s);
        }
    });
    fib(NUMBER);

    r.abort();
```

### Spawn blocking sleep loop and call fib

```rust
    let r = tokio::task::spawn_blocking(|| loop {
        std::thread::sleep(std::time::Duration::from_millis(20));
    });

    fib(NUMBER);

    r.abort();
```

```
DOES NOT HALT
```

### Spawn blocking print loop and call fib

```rust
    let r = tokio::task::spawn_blocking(|| loop {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(1000)
            .map(char::from)
            .collect();
        println!("{}", s);
    });

    fib(NUMBER);

    r.abort();
```

```
DOES NOT HALT
```
