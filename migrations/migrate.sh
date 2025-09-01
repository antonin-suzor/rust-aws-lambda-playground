function handler () {
    mig_version=$(cat current_version.txt)
    echo "mig_version: $mig_version"
    echo "migrate info:"
    sqlx migrate info --no-dotenv --source sql
    echo ""
    echo "migrate run:"
    sqlx migrate run --no-dotenv --source sql --target-version $mig_version
}
