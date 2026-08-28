FROM node:22-alpine AS builder
WORKDIR /app

COPY package.json package-lock.json ./
RUN npm install

COPY tsconfig.json tsconfig.app.json tsconfig.node.json vite.config.ts index.html ./
COPY public ./public
COPY src ./src

RUN npm run build

FROM nginx:alpine

# nginx's own entrypoint starts as root specifically so it can bind port 80 (a
# privileged port), then drops its worker processes to the unprivileged "nginx"
# user on its own -- unlike web_server, we don't add our own USER here.
COPY --from=builder /app/dist /usr/share/nginx/html

# Both variants are shipped in the image; select-nginx-config.sh (run automatically by
# nginx:alpine's own entrypoint mechanism, since it lives in /docker-entrypoint.d/) picks
# whichever one is valid at container start. nginx refuses to start at all if a
# server block references a certificate file that doesn't exist yet, so on a brand new
# deploy -- before the first certbot run has produced one -- it falls back to the
# HTTP-only config instead of crash-looping.
COPY nginx.http-only.conf /etc/nginx/conf.available/http-only.conf
COPY nginx.https.conf /etc/nginx/conf.available/https.conf
COPY select-nginx-config.sh /docker-entrypoint.d/40-select-nginx-config.sh
RUN chmod +x /docker-entrypoint.d/40-select-nginx-config.sh

EXPOSE 80 443
