import { randomUUID } from "crypto";
import { FastifyInstance } from "fastify";
import { z } from "zod";

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


export default async function(app: FastifyInstance) {


}
