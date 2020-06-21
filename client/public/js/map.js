import { Vector } from "./vector.js";

export class Tile {
	constructor(type) {
		this.type = type;
	}
}

export class GameMap {
	constructor(width, height, tiles) {
		this.width = width;
		this.height = height;
		this.tiles = tiles;
	}

	getTile(vect) {
		return this.tiles[Math.floor(vect.y)][Math.floor(vect.x)];
	}
}

export function parseMap(string) {
	let lines = string.split("\n");

	const width = Number(lines.shift());
	const height = Number(lines.shift());
	const numTypes = Number(lines.shift());
	if (isNaN(width) || isNaN(height) || isNaN(numTypes)) {
		throw new Error("Invalid map");
	}

	let types = {};
	for (let i = 0; i < numTypes; i++) {
		const parts = lines.shift().split("=").filter(x => x != "");
		if (parts.length != 2 || parts[0].length != 1) {
			throw new Error("Invalid map");
		}
		if (parts[0] in types) {
			throw new Error("Invalid map");
		}
		types[parts[0]] = parts[1];
	}

	let columns = [];
	for (let i = 0; i < height; i++) {
		const line = lines.shift();
		if (line === undefined) {
			throw new Error("Invalid map");
		}

		const keys = line.split("");
		if (keys.length != width) {
			throw new Error("Invalid map");
		}
		let row = [];
		for (const key of keys) {
			if (key in types) {
				const type = types[key];
				row.push(new Tile(type));
			} else {
				throw new Error("Invalid map");
			}
		}

		columns.push(row);
	}

	return new GameMap(width, height, columns);
}
