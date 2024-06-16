import { FastifyInstance } from "fastify";

export default async function upload_routers(app: FastifyInstance) {
    app.post('/api/upload', async (request, reply) => {
        const files = request.files();

        for await (const file of files) {

        }

        return reply.status(200);
    })
}
