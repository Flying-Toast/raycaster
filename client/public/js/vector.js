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

	mult(v) {
		this.x *= v.x;
		this.y *= v.y;
	}

	times(v) {
		return new Vector(this.x * v.x, this.y * v.y);
	}

	perp() {
		return new Vector(-this.y, this.x);
	}

	opposite() {
		return new Vector(-this.x, -this.y);
	}
}
