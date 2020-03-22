import { parseMap } from "./map.js";

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

console.log(map)
