import { defineConfig } from 'vite'
// @ts-ignore
import fs from 'fs'
// @ts-ignore
import path from 'path'
import {ViteDevServer} from "vite";
import  { IncomingMessage, ServerResponse } from 'http'

// Custom middleware to serve wasm files with the correct MIME type
const wasmMiddleware = () => {
    return {
        name: 'wasm-middleware',
        configureServer(server: ViteDevServer) {
            server.middlewares.use((req: IncomingMessage, res: ServerResponse, next: () => void) => {
                if (req.url && req.url.endsWith('.wasm')) {
                    const wasmPath = path.join('./node_modules/snake-wasm', path.basename(req.url))
                    const wasmFile = fs.readFileSync(wasmPath)
                    res.setHeader('Content-Type', 'application/wasm')
                    res.end(wasmFile)
                    return
                }
                next()
            })
        }
    }
}

export default defineConfig({
    plugins: [wasmMiddleware()],
    esbuild: {
        supported: {
            'top-level-await': true
        }
    }
})