import { sql } from "./lib/postgres.js"

async function setup() {
    await sql /*sql*/`
        CREATE TABLE IF NOT EXISTS short_links (
        id SERIAL PRIMARY KEY,
        code TEXT UNIQUE,
        original_url TEXT,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )`

    await sql /*sql*/`
       create table if not exists itens (
        id serial primary key,
        text_value text not null,
        created_at timestamp default CURRENT_TIMESTAMP
       )
    `

    await sql.end()

    console.log('setup done')
}

setup()
