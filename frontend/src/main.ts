import './style.css'
import Game from "./Game.ts"

const game = new Game(1000, 1000)
game.setShowPerformanceInfo()
await game.run();

