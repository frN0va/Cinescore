FROM node:22-alpine

WORKDIR /app

COPY package*.json ./

RUN npm install

COPY ./src ./src
COPY ./public ./public
COPY ./tsconfig* ./
COPY vite.config.ts ./
COPY index.html ./

RUN npm run build

# The build output will be in /app/dist, which will be shared with Nginx container using the docker-compose
