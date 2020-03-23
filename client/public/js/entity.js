let nextID = 0;

export class Entity {
	constructor(loc, dir, speed) {
		this.loc = loc;
		this.dir = dir;
		this.speed = speed;
		this.id = nextID++;
	}

	move(dt, dir) {
		this.loc.add(dir.stimes(this.speed * dt));
	}
}
