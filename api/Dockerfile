FROM node:20

WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
RUN npm run build
EXPOSE 3000
ENV API_ADDRESS=0.0.0.0 API_PORT=3000
CMD npm run start
