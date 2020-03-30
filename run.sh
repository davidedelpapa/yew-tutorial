#!/bin/sh

if [ -f .serverpid ]; then
    echo "Server already running, or in an inconsistent state"
    exit 1
fi

build(){
    wasm-pack build --target web
}

pack(){
   rollup ./main.js --format iife --file ./pkg/bundle.js 
}

run(){
    python3 -m "http.server" "8080" &
    PID=$!
    echo $PID > .serverpid
}

build && pack && run