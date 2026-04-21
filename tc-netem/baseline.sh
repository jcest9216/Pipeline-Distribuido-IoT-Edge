#!/bin/bash

INTERFAZ="eth0"

echo "Limpiando reglas tc en $INTERFAZ..."

sudo tc qdisc del dev $INTERFAZ root 2>/dev/null

echo "Red restaurada (baseline)"
