import { cfg } from "./config.js";

let up = false;
let down = false;
let left = false;
let right = false;
let angle = 0;

addEventListener("keydown", function(e) {
	keyEvent(true, e);
});

addEventListener("keyup", function(e) {
	keyEvent(false, e);
});

addEventListener("mousemove", function(e) {
	let a = (e.clientX / innerWidth * cfg.mouseSensitivity) * Math.PI*2;
	angle = a % Math.PI*2;
});

function keyEvent(setState, e) {
	switch (e.key.toLowerCase()) {
		case "w":
			up = setState;
			break;
		case "a":
			left = setState;
			break;
		case "s":
			down = setState;
			break;
		case "d":
			right = setState;
			break;
	}
}

export function getInput() {
	return {
		up: up,
		down: down,
		left: left,
		right: right,

		rotation: angle
	};
}
