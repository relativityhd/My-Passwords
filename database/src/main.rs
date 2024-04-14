use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

use serde::Serialize;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Scope;
use surrealdb::Surreal;

const HOST: &str = "http://localhost:8000";
const USER: &str = "root";
const PASSWORD: &str = "root";
const NAMESPACE: &str = "accounts";
const DATABASE: &str = "dev";

#[derive(Serialize)]
struct Credentials<'a> {
    email: &'a str,
    username: &'a str,
    password: &'a str,
}

fn import_surreal_file(file: &str) {
    println!("Importing file: {}", file);

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args([
                "/C", "surreal", "import", "-e", HOST, "-u", USER, "-p", PASSWORD, "--ns",
                NAMESPACE, "--db", DATABASE, file,
            ])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .args([
                "-c", "surreal", "import", "-e", HOST, "-u", USER, "-p", PASSWORD, "--ns",
                NAMESPACE, "--db", DATABASE, file,
            ])
            .output()
            .expect("failed to execute process")
    };
    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

fn setup_db() {
    println!(
        "Setting up database with user: {}, password: {}, namespace: {}, database: {}",
        USER, PASSWORD, NAMESPACE, DATABASE
    );
    let p = Path::new("datamodel/000_namespace.surql");
    import_surreal_file(p.as_os_str().to_str().unwrap());

    let p = Path::new("datamodel").read_dir().unwrap();
    for entry in p {
        let file = entry.unwrap().path();
        if file.file_stem().unwrap() == "000_namespace" {
            continue;
        }
        import_surreal_file(file.as_os_str().to_str().unwrap());
    }

    let p = Path::new("logic").read_dir().unwrap();
    for entry in p {
        let file = entry.unwrap().path();
        if file.file_stem().unwrap() == "000_roadmap" {
            continue;
        }
        import_surreal_file(file.as_os_str().to_str().unwrap());
    }
}

async fn setup_test_user() {
    println!("Setting up test-user");

    let db = Surreal::new::<Ws>("127.0.0.1:8000")
        .await
        .expect("Failed to connect to database");
    db.use_ns(NAMESPACE)
        .use_db(DATABASE)
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

async fn setup_test_data() {
    println!("Setting up data");

    let db = Surreal::new::<Ws>("127.0.0.1:8000")
        .await
        .expect("Failed to connect to database");
    db.use_ns(NAMESPACE)
        .use_db(DATABASE)
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

        LET $msacc = fn::create_supersecure_account('tobiashoelzer@hotmail.com', 'Tech', 1111, 10, 26, 'Microsoft', '3MTMwMzQ3ODU', 'https://microsoft.com', none, $homebucket, $msauth);
        fn::create_secure_account('tobiashoelzer@hotmail.com', 'Games', 'Faceit', none, none, none, $homebucket, none);
        fn::create_secure_account('tobiashoelzer@hotmail.com', 'Social', 'Instagram', none, none, none, $homebucket, none);
        fn::create_sso_account($msacc, 'GitHub', none, 'https://github.com', ['GutHub', 'Repos'], $homebucket, none);

        fn::create_secure_account('tobiashoelzer@work.com', 'Tech', 'Slack', none, none, none, $workbucket, none);
    ";
    db.query(sql).await.unwrap();
}

async fn wipe_db() {
    println!("Wiping database");

    let db = Surreal::new::<Ws>("127.0.0.1:8000")
        .await
        .expect("Failed to connect to database");
    db.use_ns(NAMESPACE)
        .use_db(DATABASE)
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
    setup_db();
    setup_test_user().await;
    wipe_db().await;
    setup_test_data().await;
}
