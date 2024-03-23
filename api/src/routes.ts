import { randomUUID } from "crypto";
import { FastifyInstance } from "fastify";
import { z } from "zod";
import { redis } from "./lib/redis";

// app.get('/:code', async (request, reply) => {
//     const getLinkSchema = z.object({
//         code: z.string().min(3),
//     })
//     const { code } = getLinkSchema.parse(request.params)
//     const result = await sql /*sql*/ `
//         select id, original_url
//           from short_links 
//          where code = ${code}
//     `
//     if (result.length === 0)
//         return reply.status(400).send({ message: 'Link not found' })
//     const link = result[0]
//     await redis.zIncrBy('metrics', 1, String(link.id))
//     return reply.redirect(301, link.original_url)
// })

// app.post('/api/links', async (request, reply) => {
//     const createLinkSchema = z.object({
//         code: z.string().min(3),
//         url: z.string().url(),
//     })

//     const { code, url } = createLinkSchema.parse(request.body)

//     try {
//         const result = await sql /*sql*/ `
//             INSERT INTO short_links (code, original_url)
//             VALUES (${code}, ${url})
//             RETURNING id            
//         `

//         const link = result[0]

//         return reply.status(201).send({ shortLinkId: link.id })
//     } catch (err) {
//         if (err instanceof postgres.PostgresError) {
//             if (err.code === '23505') {
//                 return reply.status(400).send({ message: 'Duplicated code!' })
//             }
//         }

//         console.log(err)

//         return reply.status(500).send({ message: 'Internal server error.' })
//     }
// })


export default async function (app: FastifyInstance) {
    app.post('/api', async (request, reply) => {
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

    app.put('/api/:guid', async (request, reply) => {
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

    app.get('/api/:guid', async (request, reply) => {
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