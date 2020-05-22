#!/bin/sh

stop(){
    if [ -f .serverpid ]; then
        kill $(cat .serverpid)
        rm .serverpid
        echo Server stopped.
    else
        echo Could not find the server PID. Is it running?
    fi
    if [ -f .serverpid.rollup ]; then
        kill $(cat .serverpid.rollup)
        rm .serverpid.rollup
        echo Hot-Reload server stopped.
    else
        echo Could not find the rot-reload server PID. Is it running?
    fi
}

stop
