#!/bin/bash

INTERFAZ="eth0"

echo "Aplicando limitación de red (512kbit + 50ms)..."

sudo tc qdisc add dev $INTERFAZ root netem rate 512kbit delay 50ms

echo "Limitación aplicada"
