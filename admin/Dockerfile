FROM node:18 AS build

WORKDIR /usr/src/app

COPY package.json package-lock.json /usr/src/app/

RUN npm install

COPY . /usr/src/app

RUN npm run build

FROM nginx:1.24

COPY nginx/default.conf /etc/nginx/conf.d/default.conf
COPY nginx/upstream_production.conf /etc/nginx/conf.d/upstream.conf
COPY --from=build /usr/src/app/dist /usr/share/nginx/html
