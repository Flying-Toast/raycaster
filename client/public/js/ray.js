import { Vector } from "./vector.js";

export function angleToDir(angle) {
	return new Vector(Math.cos(angle), Math.sin(angle));
}

export function castRay(map, loc, dir) {
	// location within the tile
	const innerLoc = loc.smod(1);

	const absoluteXDir = dir.x < 0 ? -1 : 1;
	const absoluteYDir = dir.y < 0 ? -1 : 1;

	// x distance until first x gridline
	const untilX = absoluteXDir * (dir.x < 0 ? innerLoc.x : 1 - innerLoc.x);
	const untilY = absoluteYDir * (dir.y < 0 ? innerLoc.y : 1 - innerLoc.y);

	// how much the x component of dir needs to be multiplied by to reach the first x gridline
	const xRatio = untilX / dir.x;
	const yRatio = untilY / dir.y;

	// distance along direction vector to first gridlines
	const alongUntilX = Math.hypot(untilX, xRatio * dir.y);
	const alongUntilY = Math.hypot(untilY, yRatio * dir.x);

	// distance along direction vector between x gridline intersections
	const alongBetweenX = Math.hypot(1, dir.y / dir.x);
	const alongBetweenY = Math.hypot(1, dir.x / dir.y);

	// cumulative distance along direction from loc to the next unchecked possible intersection with an x gridline
	let xDistAlong = alongUntilX;
	let yDistAlong = alongUntilY;

	let currentLoc = loc.clone();

	let hitXWall;

	while (map.getTile(currentLoc).type != "wall") {
		if (xDistAlong < yDistAlong) {
			xDistAlong += alongBetweenX;
			currentLoc.x += absoluteXDir;
			hitXWall = true;
		} else {
			yDistAlong += alongBetweenY;
			currentLoc.y += absoluteYDir;
			hitXWall = false;
		}
	}

	const dist = hitXWall ? xDistAlong - alongBetweenX : yDistAlong - alongBetweenY;

	return {
		// length of the ray
		distance: dist,
		// vector where the ray hit the first wall
		intersection: loc.plus(dir.stimes(dist)),
		// is the intersection on a west/east side?
		isX: hitXWall
	};
}
