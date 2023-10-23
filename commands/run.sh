cd ..
if [ $1 = "web" ]; then
    cd web
    python3 -m http.server
else
    cargo run
fi
