FROM node:26.12.1-slim@sha256:80eccfd7e3fa8663c84881efb6b28c7411672a337aec1769f0fb9e3adc4a4d8b

COPY . /action
WORKDIR /action

RUN npm install --production

ENTRYPOINT ["node", "/action/main.js"]
