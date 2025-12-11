import './style.css'
import Game from "./Game.ts"

const game = new Game(1000, 1000, true)
game.setShowPerformanceInfo()
await game.run();

