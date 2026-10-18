FROM node:24.21-slim@sha256:51b1100cc2a83d370c6a60952e3f2989c8a43159d0e38586e090f3b3326efefd

COPY . /action
WORKDIR /action

RUN npm install --production

ENTRYPOINT ["node", "/action/main.js"]
