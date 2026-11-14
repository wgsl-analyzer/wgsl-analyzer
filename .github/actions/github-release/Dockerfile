FROM node:26.13.0-slim@sha256:6f3406bb40734a2cbe3c78c3a5d9a467113b41b7a4c653a8d0534a276c67afdf

COPY . /action
WORKDIR /action

RUN npm install --production

ENTRYPOINT ["node", "/action/main.js"]
