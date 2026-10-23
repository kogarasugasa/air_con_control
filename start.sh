#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "$(readlink -f "$0")")" && pwd)"
EXE_FILE="${SCRIPT_DIR}/target/debug/air_con_control"
SEND_RECV="$1"
PROFILE="$2"
REVERSE="$3"

$EXE_FILE $SEND_RECV $PROFILE $REVERSE