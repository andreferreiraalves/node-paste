import cors from '@fastify/cors';
import 'dotenv/config';
import fastify from "fastify";
import routers from './routes';

const { API_PORT = '3000', API_ADDRESS = 'localhost' } = process.env

const app = fastify()

app.register(cors, {
    allowedHeaders: '*',
    origin: '*',
});

app.register(routers)

app.listen({
    host: API_ADDRESS,
    port: parseInt(API_PORT),
}).then(() => {
    console.log(`server running on port ${API_PORT}`)
})