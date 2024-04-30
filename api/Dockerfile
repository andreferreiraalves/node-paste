FROM node:20

WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
RUN npm run build

ENV API_ADDRESS=0.0.0.0 API_PORT=4000
CMD npm run start
