import cors from '@fastify/cors';
import multipart from '@fastify/multipart'

import 'dotenv/config';
import fastify from "fastify";
import upload_routers from './routers/upload_routers.js';
import message_routes from './routers/message_routes.js';

const { API_ADDRESS = 'localhost' } = process.env
const API_PORT = 4000

const app = fastify()

app.register(cors, {
    allowedHeaders: '*',
    origin: '*',
});

app.register(multipart);

app.register(upload_routers)
app.register(message_routes);

const start = async () => {
    try {
        await app.listen({ host: API_ADDRESS, port: API_PORT })
        console.log(`server running on port ${API_PORT}`)
    } catch (err) {
        app.log.error(err)
        process.exit(1)
    }
}

start()
