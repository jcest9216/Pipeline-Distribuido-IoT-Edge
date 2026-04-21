#!/bin/bash

INTERFAZ="eth0"

echo "Aplicando pérdida de paquetes (8%)..."

sudo tc qdisc add dev $INTERFAZ root netem loss 8%

echo "Pérdida aplicada"
