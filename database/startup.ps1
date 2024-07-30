$user = $args[0]
$pass = $args[1]

# Define the namespace and database
$ns = "accounts"
$db = "dev"

echo $user $pass $ns $db

# cat "database\datamodel\000_namespace.surql" | surreal sql --multi -u $user -p $pass

# Get the list of files in the database directory
$files = @(
    "001_user",
    "002_bucket",
    "003_twofactor",
    "004_account",
    "005_secure_account",
    "006_supersecure_account",
    "007_sso_account",
    "008_legacy_account"
) # Get-ChildItem -Path "database" -Filter "*.surql"

# Loop over the files and execute the import command for each one
foreach ($file in $files) {
    $f = ".\database\datamodel\" + $file + ".surql"
    #cat $f | surreal sql --multi -u $user -p $pass --namespace $ns --database $db
    echo $f
    surreal import -e http://localhost:8000 -u $user -p $pass --ns $ns --db $db $f
}

# Get the list of files in the database directory
$files_logic = @(
    "000_user_logic",
    "001_account_logic",
    "002_bucket_logic",
    "003_twofactor_logic"
) # Get-ChildItem -Path "database" -Filter "*.surql"

# Loop over the files and execute the import command for each one
foreach ($file in $files_logic) {
    $f = ".\database\logic\" + $file + ".surql"
    echo $f
    #cat $f | surreal sql --multi -u $user -p $pass --namespace $ns --database $db
    surreal import -e http://localhost:8000 -u $user -p $pass --ns $ns --db $db $f
}
