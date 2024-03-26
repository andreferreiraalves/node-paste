import 'dotenv/config';
import fastify from "fastify";
import routers from './routes';

const PORT = process.env.API_PORT

const app = fastify()

app.register(require('@fastify/cors'), (instance) => {
    return (req: any, callback: any) => {
        const corsOptions = {
            origin: true
        };

        // do not include CORS headers for requests from localhost
        if (/^localhost$/m.test(req.headers.origin)) {
            corsOptions.origin = false
        }

        corsOptions.origin = false

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