import { Vector } from "./vector.js";
import { Entity } from "./entity.js";

export class Game {
	constructor(map) {
		this.map = map;
		this.entities = new Map();
		this.self = new Entity(new Vector(8.5, 8.5), new Vector(1, 0));
	}

	update(dt) {

	}

	addEntity(entity) {
		this.entities.set(entity.id, entity);
	}

	removeEntity(entity) {
		this.entities.delete(entity.id);
	}
}
