use rand::{distributions::Alphanumeric, Rng};

const NUMBER: u128 = 50;

fn fib(n: u128) -> u128 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        println!("test-tokio needs 3 arguments to run.");
        return Ok(());
    }

    match (args[1].as_str(), args[2].as_str(), args[3].as_str()) {
        ("normal", "spawn", "print") => spawn_with_print().await,
        ("normal", "spawn", "sleep") => spawn_with_sleep().await,
        ("normal", "spawn_blocking", "print") => spawn_blocking_with_print().await,
        ("normal", "spawn_blocking", "sleep") => spawn_blocking_with_sleep().await,
        ("reversed", "spawn", "print") => reversed_spawn_with_print().await,
        ("reversed", "spawn", "sleep") => reversed_spawn_with_sleep().await,
        ("reversed", "spawn_blocking", "print") => reversed_spawn_blocking_with_print().await,
        ("reversed", "spawn_blocking", "sleep") => reversed_spawn_blocking_with_sleep().await,
        (_, _, _) => Err(color_eyre::eyre::eyre!("Incorrect arguments")),
    }
}

async fn reversed_spawn_blocking_with_sleep() -> color_eyre::Result<()> {
    let r = tokio::task::spawn_blocking(|| loop {
        std::thread::sleep(std::time::Duration::from_millis(20));
    });

    fib(NUMBER);

    r.abort();
    Ok(())
}

async fn reversed_spawn_blocking_with_print() -> color_eyre::Result<()> {
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
    Ok(())
}

async fn reversed_spawn_with_sleep() -> color_eyre::Result<()> {
    let r = tokio::task::spawn(async {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
        }
    });

    fib(NUMBER);

    r.abort();
    Ok(())
}

async fn reversed_spawn_with_print() -> color_eyre::Result<()> {
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
    Ok(())
}

async fn spawn_blocking_with_sleep() -> color_eyre::Result<()> {
    let r = tokio::task::spawn_blocking(|| fib(NUMBER));

    while !r.is_finished() {
        tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
    }

    let r = r.await?;
    println!("{r}");
    Ok(())
}

async fn spawn_blocking_with_print() -> color_eyre::Result<()> {
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
    println!("{r}");
    Ok(())
}
async fn spawn_with_sleep() -> color_eyre::Result<()> {
    let r = tokio::task::spawn(async { fib(NUMBER) });

    while !r.is_finished() {
        tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
    }

    let r = r.await?;
    println!("{r}");
    Ok(())
}

async fn spawn_with_print() -> color_eyre::Result<()> {
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
    println!("{r}");
    Ok(())
}
