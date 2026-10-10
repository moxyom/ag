import vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vite'
import { viteSingleFile } from 'vite-plugin-singlefile'

// https://vite.dev/config/
export default defineConfig(({ mode }) => {
    const isAdmin = mode === 'admin'

    return {
        plugins: [
            vue(),
            viteSingleFile(),
        ],

        build: {
            outDir: isAdmin ? 'dist/admin' : 'dist/client',
            emptyOutDir: true,

            rollupOptions: {
                input: isAdmin ? 'admin.html' : 'client.html',
            },
        },
    }
})
