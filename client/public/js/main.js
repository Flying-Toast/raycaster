import { parseMap } from "./map.js";
import { Game } from "./game.js";
import { Renderer } from "./render.js";
import { runGameLoop } from "./gameloop.js";

let map = parseMap(
`20
20
2
X=wall
 =air
XXXXXXXXXXXXXXXXXXXX
X                  X
X                  X
X                  X
X              XXX X
X               XX X
X    XXXX        X X
X                  X
X    XX            X
X     X            X
X     X            X
X                  X
X X                X
X X          XX    X
X X          XX    X
X XX               X
X  X               X
X                  X
X                  X
XXXXXXXXXXXXXXXXXXXX`
);

let game = new Game(map);
let renderer = new Renderer(document.querySelector("#canvas"));
runGameLoop(game, renderer);
