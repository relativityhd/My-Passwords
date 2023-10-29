$user = $args[0]
$pass = $args[1]

# Define the namespace and database
$ns = "accounts"
$db = "dev"

echo $user $pass $ns $db

cat "database\datamodel\000_namespace.surql" | surreal sql --multi -u $user -p $pass

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
    $f = "database\datamodel\" + $file + ".surql"
    cat $f | surreal sql --multi -u $user -p $pass --namespace $ns --database $db
    # surreal import --ns $ns --db $db $f
}
