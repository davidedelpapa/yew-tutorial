#!/bin/sh

stop(){
    if [ -f .serverpid ]; then
        kill $(cat .serverpid) && rm .serverpid
        echo Server stopped.
    else
        echo Could not find the server PID. Is it running?
    fi
}

stop
