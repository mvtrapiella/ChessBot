#!/bin/sh
set -e

if [ -f /etc/letsencrypt/live/atlas-chessbot.duckdns.org/fullchain.pem ]; then
    cp /etc/nginx/conf.available/https.conf /etc/nginx/conf.d/default.conf
else
    cp /etc/nginx/conf.available/http-only.conf /etc/nginx/conf.d/default.conf
fi
