FROM node:24.21-slim@sha256:5cbc7caba8c2c0f0bca675d1b61b9f2857e1cf1853c6164ee9dd409501a936e7

COPY . /action
WORKDIR /action

RUN npm install --production

ENTRYPOINT ["node", "/action/main.js"]
