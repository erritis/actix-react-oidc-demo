FROM node:14.17.6-alpine AS build

RUN apk --no-cache add curl

ENV PATH=/web/node_modules/.bin:$PATH

WORKDIR /app

COPY package.json yarn.lock /app/

RUN yarn install

ARG REACT_APP_KEYCLOAK_CLIENT_ID=react
ARG REACT_APP_KEYCLOAK_URL
ARG REACT_APP_BACKEND_URL

ENV REACT_APP_ENVIRONMENT_NAME=development
ENV NODE_ENV=development
ENV PORT=3000

EXPOSE 3000

COPY . /app/

RUN yarn build

CMD [ "npx", "serve", "build" ]
