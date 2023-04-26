echo "$(pidof 3000)"
kill "$(pidof 3000)" # kill any running instances to free up port
3000