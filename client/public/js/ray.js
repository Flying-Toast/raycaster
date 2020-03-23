import { Vector } from "./vector.js";

export function angleToDir(angle) {
	return new Vector(Math.cos(angle), Math.sin(angle));
}
