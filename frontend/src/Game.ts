import init, {GameState, GameWasm} from "snake-wasm"

class Game {
    private readonly width: number
    private readonly height: number

    private readonly ctx: CanvasRenderingContext2D | null

    private showPerformanceInfo: boolean = false
    private wasm: any
    private wasmGame: GameWasm | null = null
    private imgData: ImageData | null | undefined

    private lastTime: DOMHighResTimeStamp
    private frameCount: number = 0
    private fps: number = 0
    private lastFpsUpdate: DOMHighResTimeStamp
    private delta_time: number = 0

    constructor(width: number, height: number, showPerformanceInfo: boolean = false) {
        this.height = height
        this.width = width
        const canvas = this.init()
        this.ctx = canvas.getContext('2d')
        if (!this.ctx) {
            throw new Error('Failed to get 2D context')
        }

        this.lastTime = performance.now()
        this.lastFpsUpdate = performance.now()
        this.showPerformanceInfo = showPerformanceInfo
    }

    private init(): HTMLCanvasElement {
        const canvas = document.createElement('canvas')
        canvas.width = this.width
        canvas.height = this.height
        document.body.appendChild(canvas)
        init().then((wasmModule: any) => {
            this.wasm = wasmModule
            if (!this.wasm) {
                throw new Error('Failed to load WASM module')
            }
            this.wasmGame = new GameWasm(this.width, this.height)
            if (!this.wasmGame) {
                throw new Error('Failed to initialize WASM game')
            }
            this.imgData = this.setupImageData()

            document.body.addEventListener('keydown', (e: KeyboardEvent) => {
                // console.log(e.code)
                this.wasmGame?.key_down(e.code)
            })
        }).catch(err => {
            console.error('Error initializing WASM module:', err)
        })
        return canvas
    }

    private setupImageData(): ImageData | null {
        if (!this.wasm || !this.wasmGame) return null
        const memory = this.wasm.memory as WebAssembly.Memory
        const ptr = this.wasmGame.get_screen_buffer()
        const buf = new Uint8ClampedArray(memory.buffer, ptr, this.width * this.height * 4)
        return new ImageData(buf, this.width, this.height)
    }

    public setShowPerformanceInfo(show: boolean = true): void {
        this.showPerformanceInfo = show
    }

    public async run(): Promise<void> {
        while (true) {
            this.update()
            await this.render()
        }
    }

    private update(): void {
        // // FPS calculation
        const now = performance.now()
        this.frameCount++
        this.delta_time = now - this.lastTime
        this.lastTime = now
        if (now - this.lastFpsUpdate >= 1000) {
            this.fps = this.frameCount
            this.frameCount = 0
            this.lastFpsUpdate = now
        }

        if (this.wasmGame) {
            this.wasmGame.update(this.delta_time)
        }
    }

    private renderTextLayer(): void {
        if (!this.ctx || !this.wasmGame) {
            return
        }
        const prevAlign = this.ctx.textAlign
        const prevBaseline = this.ctx.textBaseline
        if (this.showPerformanceInfo) {
            const fontSize = this.height / 50
            this.ctx.fillStyle = 'white'
            this.ctx.font = `${fontSize}px Arial`
            this.ctx.textAlign = 'left'
            this.ctx.textBaseline = 'top'
            this.ctx.fillText(`FPS: ${this.fps}, Delta: ${this.delta_time}, State: ${this.wasmGame.get_game_state()}`, 20, 30)
        }

        const fontSize = this.width / 20
        const gameState = this.wasmGame.get_game_state()

        this.ctx.textAlign = 'center'
        this.ctx.textBaseline = 'middle'
        if (GameState.GameOver === gameState) {
            this.ctx.fillStyle = 'rgba(0, 0, 0, 0.5)'
            this.ctx.fillRect(0, 0, this.width, this.height)
            this.ctx.fillStyle = 'red'
            this.ctx.font = `${fontSize}px Arial`
            this.ctx.fillText('Game Over', this.width / 2, this.height / 2)
        } else if (GameState.Paused === gameState) {
            this.ctx.fillStyle = 'rgba(0, 0, 0, 0.5)'
            this.ctx.fillRect(0, 0, this.width, this.height)
            this.ctx.fillStyle = 'yellow'
            this.ctx.font = `${fontSize}px Arial`
            this.ctx.fillText('Paused', this.width / 2, this.height / 2)
        }
        this.ctx.textAlign = prevAlign
        this.ctx.textBaseline = prevBaseline
    }

    private async render(): Promise<void> {
        if (!this.ctx || !this.wasmGame || !this.imgData) {
            await new Promise(requestAnimationFrame)
            return
        }

        this.wasmGame.render()
        this.ctx.putImageData(this.imgData, 0, 0)
        this.renderTextLayer();
        await new Promise(requestAnimationFrame)
    }
}

export default Game