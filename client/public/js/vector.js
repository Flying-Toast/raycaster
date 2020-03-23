export class Vector {
	constructor(x, y) {
		this.x = x;
		this.y = y;
	}

	add(v) {
		this.x += v.x;
		this.y += v.y;
	}

	plus(v) {
		return new Vector(this.x + v.x, this.y + v.y);
	}

	smult(s) {
		this.x *= s;
		this.y *= s;
	}

	stimes(s) {
		return new Vector(this.x * s, this.y * s);
	}

	perp() {
		return new Vector(-this.y, this.x);
	}

	opp() {
		return new Vector(-this.x, -this.y);
	}
}
