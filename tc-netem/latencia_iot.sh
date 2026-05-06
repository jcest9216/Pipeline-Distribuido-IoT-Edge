#!/bin/bash

INTERFAZ="wg0"

echo "Aplicando latencia (80ms + 20ms jitter)..."

sudo tc qdisc add dev $INTERFAZ root netem delay 80ms 20ms

echo "Latencia aplicada"
