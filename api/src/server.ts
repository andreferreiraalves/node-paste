import 'dotenv/config';
import fastify from "fastify";
import routers from './routes';

const PORT = process.env.API_PORT

const app = fastify()

app.register(require('@fastify/cors'), (instance) => {
    return (req: any, callback: any) => {
        const corsOptions = {
            // This is NOT recommended for production as it enables reflection exploits
            origin: true
        };

        // do not include CORS headers for requests from localhost
        if (req.headers.host.indexOf('localhost') != -1) {
            corsOptions.origin = false
        }

        // callback expects two parameters: error and options
        callback(null, corsOptions)
    }
})

app.register(routers)

app.listen({
    port: parseInt(PORT ?? '3000'),
}).then(() => {
    console.log(`server running on port ${PORT}`)
})