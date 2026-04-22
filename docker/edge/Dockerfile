FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    wireguard \
    iproute2 \
    iputils-ping \
    nano \
    vim \
    curl \
    iperf3 \
    netcat-openbsd \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /etc/wireguard

RUN echo '#!/bin/bash\n\
    wg genkey | tee /etc/wireguard/private.key | wg pubkey > /etc/wireguard/public.key\n\
    echo "PRIVATE KEY:"\n\
    cat /etc/wireguard/private.key\n\
    echo "PUBLIC KEY:"\n\
    cat /etc/wireguard/public.key\n\
    ' > /usr/local/bin/generar-llaves && chmod +x /usr/local/bin/generar-llaves


RUN echo "[Interface]" > /etc/wireguard/wg0.conf && \
    echo "Address = SUSTITUIR_POR_IP_ASIGNADA" >> /etc/wireguard/wg0.conf && \
    echo "PrivateKey = SUSTITUIR_POR_PRIVATE_KEY" >> /etc/wireguard/wg0.conf && \
    echo "ListenPort = 51820" >> /etc/wireguard/wg0.conf && \
    echo "" >> /etc/wireguard/wg0.conf && \
    echo "[Peer]" >> /etc/wireguard/wg0.conf && \
    echo "PublicKey = SUSTITUIR_POR_PUBLIC_KEY_DEL_COORD" >> /etc/wireguard/wg0.conf && \
    echo "Endpoint = SUSTITUIR_POR_IP_DEL_CONTENEDOR_COORD:51820" >> /etc/wireguard/wg0.conf && \
    echo "AllowedIPs = 10.0.0.1/32" >> /etc/wireguard/wg0.conf && \
    echo "PersistentKeepalive = 25" >> /etc/wireguard/wg0.conf

CMD ["sleep", "infinity"]
