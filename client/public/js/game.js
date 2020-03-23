import { cfg } from "./config.js";
import { Vector } from "./vector.js";
import { Entity } from "./entity.js";
import { getInput } from "./input.js";
import { angleToDir } from "./ray.js";

export class Game {
	constructor(map) {
		this.map = map;
		this.entities = new Map();
		this.self = new Entity(new Vector(8.5, 8.5), new Vector(1, 0), cfg.playerSpeed);
	}

	update(dt) {
		const inp = getInput();
		this.self.dir = angleToDir(inp.rotation);
		if (inp.up) {
			this.self.move(dt, this.self.dir);
		}
		if (inp.down) {
			this.self.move(dt, this.self.dir.opp());
		}
		if (inp.left) {
			this.self.move(dt, this.self.dir.perp().opp());
		}
		if (inp.right) {
			this.self.move(dt, this.self.dir.perp());
		}
	}

	addEntity(entity) {
		this.entities.set(entity.id, entity);
	}

	removeEntity(entity) {
		this.entities.delete(entity.id);
	}
}
