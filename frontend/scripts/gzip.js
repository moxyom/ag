import { readFileSync, writeFileSync } from 'node:fs'
import { constants, gzipSync } from 'node:zlib'

// Rust embeds these via include_bytes!
const files = ['dist/client/client.html', 'dist/admin/admin.html']

for (const file of files) {
    const input = readFileSync(file)
    const output = gzipSync(input, { level: constants.Z_BEST_COMPRESSION })
    writeFileSync(`${file}.gz`, output)
    console.log(`${file}.gz  ${input.length} -> ${output.length} bytes`)
}
