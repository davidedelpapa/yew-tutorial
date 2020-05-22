#!/bin/bash

help() {
    echo "Run this script to get a dev-env"
}

create_version() {
    datetime=`date +"%D %T"`
    cat > ./src/js/version.js << EOL
// Do not modify; created programmatically
export function __compiled() {
    return "${datetime}"
} 
EOL
}

build(){
    create_version
    wasm-pack build --target web
}

pack(){
    cmd="rollup -c"
    $cmd &
    PID=$!
    echo $PID > .serverpid.rollup
}

run(){
    thttp -p 8080 &
    PID=$!
    echo $PID > .serverpid
}

watch() {
    cargo watch -w ./src --ignore "*.js" --postpone -s "run.sh --reload" 
}

stop() {
    kill $(cat .serverpid)
    rm .serverpid
}

while [ "$1" != "" ]; do
    case $1 in
        -r | --reload )         stop
                                build
                                run
                                exit
                                ;;
        -h | --help )           help
                                exit
                                ;;
    esac
    shift
done


if [ -f .serverpid ]; then
    echo "Server already running, or in an inconsistent state"
    exit 1
fi

build && pack && run && watch