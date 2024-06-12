import { FastifyInstance } from "fastify";

export default async function upload_routers(app: FastifyInstance) {
    app.post('/api/upload', async (requrest, reply) => {
        const data = await requrest.file();

        console.log(data);

        return reply.status(200);
    })
}