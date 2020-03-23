let nextID = 0;

export class Entity {
	constructor(loc, dir) {
		this.loc = loc;
		this.dir = dir;
		this.id = nextID++;
	}
}
