cd ..
if [ $1 = "web" ]; then
    echo build web only 
    wasm-pack build --target no-modules --no-pack --out-dir web
elif [ $1 = "all" ]; then
    echo build all
    cargo build
    wasm-pack build --target no-modules --no-pack --out-dir web
else
    echo build local only
    cargo build
fi