import { createClient } from 'redis'

const { REDIS_SERVER = 'localhost', REDIS_PORT = 6379 } = process.env

export const redis = createClient({
      url: `redis://:docker@${REDIS_SERVER}:${REDIS_PORT}`,
})

redis.connect()