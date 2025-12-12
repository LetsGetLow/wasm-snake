import {GameEvent} from "snake-wasm";

class AudioManger {
    private static instance: AudioManger
    private readonly audioFolder: string
    private ctx: AudioContext
    private audioBuffers: Map<GameEvent, AudioBuffer>
    private backgroundMusicBuffer: AudioBuffer | null = null
    private backgroundSource: AudioBufferSourceNode | null = null
    private backgroundBufferOffset: number = 0;

    private constructor(audioFolder: string) {
        this.audioFolder = audioFolder
        this.ctx = new AudioContext()
        this.audioBuffers = new Map<GameEvent, AudioBuffer>()

        const backgroundFile = `${audioFolder}/music.mp3`;
        fetch(backgroundFile)
            .then(response => response.arrayBuffer())
            .then(arrayBuffer => this.ctx.decodeAudioData(arrayBuffer))
            .then(audioBuffer => {
                this.backgroundMusicBuffer = audioBuffer;
            })
            .catch(err => console.error('Error loading background music:', err));
    }

    public static getInstance(audioFolder = 'audio/'): AudioManger {
        if (!AudioManger.instance) {
            AudioManger.instance = new AudioManger(audioFolder)
        }

        return AudioManger.instance
    }

    public async loadAudio(event: GameEvent, file: string): Promise<void> {
        const filePath = `${this.audioFolder}/${file}`;
        const response = await fetch(filePath)
        const arrayBuffer = await response.arrayBuffer()
        const audioBuffer = await this.ctx.decodeAudioData(arrayBuffer)
        this.audioBuffers.set(event, audioBuffer)
    }

    public playAudio(event: GameEvent): void {

        switch (event) {
            case GameEvent.GameOver:
                this.stopBackgroundMusic()
                break;
            case GameEvent.GamePause:
                this.pauseBackgroundMusic()
                break;
            case GameEvent.GameStart:
                this.playBackgroundMusic()
                break;
        }

        const audioBuffer = this.audioBuffers.get(event)
        if (audioBuffer) {
            const source = this.ctx.createBufferSource()
            source.buffer = audioBuffer
            source.connect(this.ctx.destination)
            source.start()
        }
    }

    private playBackgroundMusic(loop: boolean = true): void {
        if (!this.backgroundMusicBuffer) {
            return
        }

        if (this.backgroundSource) {
            return
        }

        const gainNode = this.ctx.createGain()
        gainNode.gain.value = 0.2
        gainNode.connect(this.ctx.destination)

        this.backgroundSource = this.ctx.createBufferSource()
        this.backgroundSource.buffer = this.backgroundMusicBuffer
        this.backgroundSource.loop = loop
        this.backgroundSource.connect(gainNode)
        this.backgroundSource.start(0, this.backgroundBufferOffset)
        this.backgroundBufferOffset = 0;
    }

    private pauseBackgroundMusic(): void {
        if (this.backgroundSource) {
            const elapsed = this.ctx.currentTime
            const duration = this.backgroundSource.buffer ? this.backgroundSource.buffer.duration : 0
            this.backgroundBufferOffset = elapsed % duration || 0;
            this.backgroundSource.stop()
            this.backgroundSource.disconnect()
            this.backgroundSource = null
        }
    }

    stopBackgroundMusic(): void {
        if (this.backgroundSource) {
            this.backgroundSource.stop()
            this.backgroundSource.disconnect()
            this.backgroundSource = null
        }
        this.backgroundBufferOffset = 0;
    }
}

export default AudioManger