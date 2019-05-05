#!/bin/sh
rm -rf ./load
gcc -rdynamic -o load load.c -ldl

