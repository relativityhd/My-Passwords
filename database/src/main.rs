#![allow(non_snake_case)]

use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

use serde::Serialize;

use clap::Parser;
use surrealdb::engine::remote::ws::{Ws, Wss};
use surrealdb::opt::auth::{Root, Scope};
use surrealdb::Surreal;

struct SurrealAuth {
    host: String,
    wshost: String,
    user: String,
    password: String,
    namespace: String,
    database: String,
    secure: bool,
}

#[derive(Parser)]
enum SubCommand {
    Setup,
    Migrate,
    SetupTestData,
}

// My-Accounts database setup and migration tool
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
    #[clap(long, default_value = "http://localhost:8000")]
    host: String,
    #[clap(long, default_value = "127.0.0.1:8000")]
    wshost: String,
    #[clap(long, default_value = "root")]
    user: String,
    #[clap(long, default_value = "root")]
    password: String,
    #[clap(long, default_value = "accounts")]
    namespace: String,
    #[clap(long, default_value = "dev")]
    database: String,
}

#[derive(Serialize)]
struct Credentials<'a> {
    email: &'a str,
    username: &'a str,
    password: &'a str,
}

async fn set_version(auth: &SurrealAuth, version: &str) {
    println!("Setting version");
    println!("Version: {}", version);

    let db = if auth.secure {
        Surreal::new::<Wss>(&auth.wshost)
            .await
            .expect("Failed to connect to database")
    } else {
        Surreal::new::<Ws>(&auth.wshost)
            .await
            .expect("Failed to connect to database")
    };
    db.use_ns(&auth.namespace)
        .use_db(&auth.database)
        .await
        .expect("Failed to use namespace");

    db.signin(Root {
        username: &auth.user,
        password: &auth.password,
    })
    .await
    .expect("Failed to sign in");

    let sql = format!("DEFINE PARAM $version VALUE '{}'", version);
    let res = db.query(sql).await.expect("Failed to set version");
    dbg!(&res);

    let sql = "$version";
    let found = db
        .query(sql)
        .await
        .expect("Failed to get version")
        .take::<Option<String>>(0)
        .expect("Failed to get version");
    dbg!(&found);
    let found = found.unwrap();
    if version != found {
        panic!("Version mismatch. Expected: {}, found: {}", version, found);
    }
}

fn import_surreal_file(file: &str, auth: &SurrealAuth) {
    println!("Importing file: {}", file);

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args([
                "/C",
                "surreal",
                "import",
                "-e",
                &auth.host,
                "-u",
                &auth.user,
                "-p",
                &auth.password,
                "--ns",
                &auth.namespace,
                "--db",
                &auth.database,
                file,
            ])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .args([
                "-c",
                "surreal",
                "import",
                "-e",
                &auth.host,
                "-u",
                &auth.user,
                "-p",
                &auth.password,
                "--ns",
                &auth.namespace,
                "--db",
                &auth.database,
                file,
            ])
            .output()
            .expect("failed to execute process")
    };
    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

async fn setup_db(auth: &SurrealAuth) {
    println!(
        "Setting up database with user: {}, password: ***, namespace: {}, database: {}",
        &auth.user, &auth.namespace, &auth.database
    );
    // Create namespace and database if not exists
    let db = if auth.secure {
        Surreal::new::<Wss>(&auth.wshost)
            .await
            .expect("Failed to connect to database")
    } else {
        Surreal::new::<Ws>(&auth.wshost)
            .await
            .expect("Failed to connect to database")
    };

    db.signin(Root {
        username: &auth.user,
        password: &auth.password,
    })
    .await
    .expect("Failed to sign in");

    let sql = format!(
        "DEFINE NAMESPACE {};
                      USE NAMESPACE {};
                      DEFINE DATABASE {};
                      USE DB {};
                      ",
        &auth.namespace, &auth.namespace, &auth.database, &auth.database,
    );
    let res = db
        .query(sql)
        .await
        .expect("Failed to create namespace and database");
    dbg!(&res);

    // let p = Path::new("datamodel/000_namespace.surql");
    // import_surreal_file(p.as_os_str().to_str().unwrap(), &auth);

    let p = Path::new("datamodel").read_dir().unwrap();
    for entry in p {
        let file = entry.unwrap().path();
        if file.file_stem().unwrap() == "000_namespace" {
            continue;
        }
        import_surreal_file(file.as_os_str().to_str().unwrap(), auth);
    }

    let p = Path::new("logic").read_dir().unwrap();
    for entry in p {
        let file = entry.unwrap().path();
        import_surreal_file(file.as_os_str().to_str().unwrap(), auth);
    }
    set_version(&auth, "0.2.0").await;
}

async fn migrate_db(auth: &SurrealAuth) {
    let version = env!("CARGO_PKG_VERSION");
    println!(
        "Migrating database with user: {}, password: ***, namespace: {}, database: {} to version {}",
        &auth.user, &auth.namespace, &auth.database, version
    );

    if version == "0.2.0" {
        set_version(&auth, version).await;
        println!("No migration needed for version 0.2.0");
        return;
    }

    let p = Path::new("migrate").join(format!("{}.surql", version));
    if p.exists() {
        import_surreal_file(p.as_os_str().to_str().unwrap(), auth);
        set_version(&auth, version).await;
    } else {
        println!("No migration file found for version {}", version);
    }
}

async fn setup_test_user(auth: &SurrealAuth) {
    println!("Setting up test-user");

    let db = if auth.secure {
        Surreal::new::<Wss>(&auth.wshost)
            .await
            .expect("Failed to connect to database")
    } else {
        Surreal::new::<Ws>(&auth.wshost)
            .await
            .expect("Failed to connect to database")
    };
    db.use_ns(&auth.namespace)
        .use_db(&auth.database)
        .await
        .expect("Failed to use namespace");

    let username = "Test";
    let email = "test@test.de";
    let password = "1234";

    let token = db
        .signup(Scope {
            namespace: "accounts",
            database: "dev",
            scope: "user",
            params: Credentials {
                username: &username,
                email: &email,
                password: &password,
            },
        })
        .await
        .unwrap();
    println!("Signed up!");
    println!("Token: {}", token.as_insecure_token());
    db.authenticate(token.clone()).await.unwrap();
}

async fn setup_test_data(auth: &SurrealAuth) {
    println!("Setting up data");

    let db = if auth.secure {
        Surreal::new::<Wss>(&auth.wshost)
            .await
            .expect("Failed to connect to database")
    } else {
        Surreal::new::<Ws>(&auth.wshost)
            .await
            .expect("Failed to connect to database")
    };
    db.use_ns(&auth.namespace)
        .use_db(&auth.database)
        .await
        .expect("Failed to use namespace");

    let identity = "Test";
    let password = "1234";

    let token = db
        .signin(Scope {
            namespace: "accounts",
            database: "dev",
            scope: "user",
            params: Credentials {
                username: &identity,
                email: &identity,
                password: &password,
            },
        })
        .await
        .unwrap();
    println!("Signed in!");
    println!("Token: {}", token.as_insecure_token());
    db.authenticate(token.clone()).await.unwrap();

    // Create two buckets with different colors and names
    let sql = "
        LET $workbucket = fn::create_bucket('work', '#ff0000');
        LET $homebucket = fn::create_bucket('home', '#00ff00');

        LET $msauth = fn::create_twofactor('Microsoft Authentification', 'A70');

        LET $msacc = fn::create_supersecure_account('tobiashoelzer@hotmail.com', 'Tech', '&$%$!', 13, 10, 26, 'Microsoft', '3MTMwMzQ3ODU', 'https://microsoft.com', none, $homebucket, $msauth);
        fn::create_secure_account('tobiashoelzer@hotmail.com', 'Games', 'Faceit', none, none, none, $homebucket, none);
        fn::create_secure_account('tobiashoelzer@hotmail.com', 'Social', 'Instagram', none, none, none, $homebucket, none);
        fn::create_secure_account('tobiashoelzer@hotmail.com', 'Social', 'Signal', none, none, none, none, none);
        fn::create_sso_account($msacc, 'GitHub', none, 'https://github.com', ['GutHub', 'Repos'], $homebucket, none);

        fn::create_secure_account('tobiashoelzer@work.com', 'Tech', 'Slack', none, none, none, $workbucket, none);
    ";
    db.query(sql).await.unwrap();
}

async fn wipe_db(auth: &SurrealAuth) {
    println!("Wiping database");

    let db = if auth.secure {
        Surreal::new::<Wss>(&auth.wshost)
            .await
            .expect("Failed to connect to database")
    } else {
        Surreal::new::<Ws>(&auth.wshost)
            .await
            .expect("Failed to connect to database")
    };
    db.use_ns(&auth.namespace)
        .use_db(&auth.database)
        .await
        .expect("Failed to use namespace");

    let identity = "Test";
    let password = "1234";

    let token = db
        .signin(Scope {
            namespace: "accounts",
            database: "dev",
            scope: "user",
            params: Credentials {
                username: &identity,
                email: &identity,
                password: &password,
            },
        })
        .await
        .unwrap();
    println!("Signed in!");
    println!("Token: {}", token.as_insecure_token());
    db.authenticate(token.clone()).await.unwrap();

    let sql = "
        DELETE is_legacy;
        DELETE is_secure;
        DELETE is_supersecure;
        DELETE is_secured_by;
        DELETE is_sorted_in;
        DELETE use_sso_of;
        DELETE bucket;
        DELETE twofactor;
        DELETE legacy_account;
        DELETE secure_account;
        DELETE supersecure_account;
        DELETE account;
    ";
    db.query(sql).await.unwrap();
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let auth = SurrealAuth {
        host: args.host.clone(),
        wshost: args.wshost,
        user: args.user,
        password: args.password,
        namespace: args.namespace,
        database: args.database,
        secure: args.host.starts_with("https"),
    };
    match args.subcmd {
        SubCommand::Setup => setup_db(&auth).await,
        SubCommand::Migrate => migrate_db(&auth).await,
        SubCommand::SetupTestData => {
            setup_db(&auth).await;
            setup_test_user(&auth).await;
            wipe_db(&auth).await;
            setup_test_data(&auth).await;
        }
    }
}
