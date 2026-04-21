#!/bin/bash

CONTENEDOR=$1

if [ -z "$CONTENEDOR" ]; then
  echo "Uso: ./falla_nodo.sh <nombre_contenedor>"
  exit 1
fi

echo "Deteniendo contenedor $CONTENEDOR..."

docker stop $CONTENEDOR

echo "Contenedor detenido (simulación de falla)"
