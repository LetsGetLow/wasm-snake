import './style.css'
import Game from "./Game.ts"

const game = new Game(2000, 2000, true)
game.setShowPerformanceInfo()
await game.run();

