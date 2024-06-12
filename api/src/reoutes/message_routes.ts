import { randomUUID } from "crypto";
import { FastifyInstance } from "fastify";
import { z } from "zod";
import { redis } from "../lib/redis";

export default async function message_routes(app: FastifyInstance) {
    app.post('/api/message', async (request, reply) => {
        const schema = z.object({
            message: z.string().min(3)
        })
        const { message } = schema.parse(request.body);

        const objectStore = {
            message
        }

        const key = randomUUID()

        try {
            await redis.set(key.toString(), JSON.stringify(objectStore))
        } catch (err) {
            console.log(err)
            return reply.status(500).send(err)
        }

        return reply.status(201).send({
            key,
            message,
        })
    })



    app.put('/api/message/:guid', async (request, reply) => {
        const paramSchema = z.object({
            guid: z.string()
        })
        const bodySchema = z.object({
            message: z.string().min(3)
        })

        const { guid } = paramSchema.parse(request.params)
        const { message } = bodySchema.parse(request.body)

        try {
            const objectDB = await redis.get(guid)
            if (!objectDB)
                return reply.status(400).send({
                    message: "Guid not found"
                })

            const newMessage = JSON.parse(objectDB)
            newMessage.message = message

            await redis.set(guid, JSON.stringify(newMessage))

            return reply.status(201).send(newMessage)
        }
        catch (err) {
            console.log(err)
            return reply.status(500).send(err)
        }
    })

    app.get('/api/message/:guid', async (request, reply) => {
        const paramSchema = z.object({
            guid: z.string()
        })

        const { guid } = paramSchema.parse(request.params);
        const result = await redis.get(guid);

        if (!result)
            return reply.status(404).send({
                message: "Guid not found"
            })

        const response = JSON.parse(result)
        return response
    })
}