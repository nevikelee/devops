FROM node:20-alpine

WORKDIR /app

COPY package.json .
RUN npm install

COPY index.js .

VOLUME /data

CMD ["node", "index.js"]
