import cors from '@fastify/cors';
import 'dotenv/config';
import fastify from "fastify";
import routers from './routes';

const PORT = process.env.API_PORT

const app = fastify()

app.register(cors, {
    allowedHeaders: '*',
    origin: '*',
});

app.register(routers)

app.listen({
    port: parseInt(PORT ?? '3000'),
}).then(() => {
    console.log(`server running on port ${PORT}`)
})