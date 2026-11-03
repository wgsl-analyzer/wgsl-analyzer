FROM node:26.12.1-slim@sha256:84f8a721625927ef3ccdf94509ecaea463a266d23f1b01a184061e84d3a6574d

COPY . /action
WORKDIR /action

RUN npm install --production

ENTRYPOINT ["node", "/action/main.js"]
