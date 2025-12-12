import './style.css'
import Game from "./Game.ts"

const game = new Game(1000, 1000)
// Enable performance info display
// game.setShowPerformanceInfo()

await game.run();

