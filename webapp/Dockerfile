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

EXPOSE 80
