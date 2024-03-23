import fastify from "fastify";
import routers from './routes';

const app = fastify()

app.register(routers)

app.listen({
    port: 3000
}).then(() => {
    console.log('server running on port 3000')
})