import fastify from "fastify";
import postgres from "postgres";
import z from "zod";
import { sql } from "./lisb/postgres";
import { redis } from "./lisb/redis";

const app = fastify()

app.get('/:code', async (request, reply) => {
    const getLinkSchema = z.object({
        code: z.string().min(3),
    })

    const { code } = getLinkSchema.parse(request.params)

    const result = await sql /*sql*/ `
        select id, original_url
          from short_links 
         where code = ${code}
    `

    if (result.length === 0)
        return reply.status(400).send({ message: 'Link not found' })

    const link = result[0]

    await redis.zIncrBy('metrics', 1, String(link.id))

    return reply.redirect(301, link.original_url)
})

app.get('/api/links', async () => {
    const result = sql /*sql*/`
        select id, original_url
          from short_links
          order by created_at DESC
    `

    return result
})

app.post('/api/links', async (request, reply) => {
    const createLinkSchema = z.object({
        code: z.string().min(3),
        url: z.string().url(),
    })

    const { code, url } = createLinkSchema.parse(request.body)

    try {
        const result = await sql /*sql*/ `
            INSERT INTO short_links (code, original_url)
            VALUES (${code}, ${url})
            RETURNING id            
        `

        const link = result[0]

        return reply.status(201).send({ shortLinkId: link.id })
    } catch (err) {
        if (err instanceof postgres.PostgresError) {
            if (err.code === '23505') {
                return reply.status(400).send({ message: 'Duplicated code!' })
            }
        }

        console.log(err)

        return reply.status(500).send({ message: 'Internal server error.' })
    }
})

app.get('/api/metrics', async () => {
    const result = await redis.zRangeByScoreWithScores('metrics', 0, 50)

    const metrics = result
        .sort((a, b) => b.score - a.score)
        .map(item => {
            return {
                shortLinkId: Number(item.value),
                clicks: item.score,
            }
        })

    return metrics;
})


app.listen({
    port: 3000
}).then(() => {
    console.log('server running on port 3000')
})