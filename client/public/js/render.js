import { cfg } from "./config.js";
import { castRay } from "./ray.js";

export class Renderer {
	constructor(canvas) {
		this.width = cfg.resolution.width;
		this.height = cfg.resolution.height;
		canvas.width = this.width;
		canvas.height = this.height;
		this.canvas = canvas;
		this.ctx = canvas.getContext("2d");
	}

	render(game) {
		this.render2D(game);
	}

	render2D(game) {
		this.renderMap2D(game.map);
		this.renderSelf2D(game.self);

		const ray = castRay(game.map, game.self.loc, game.self.dir).intersection;
		let ctx = this.ctx;
		ctx.strokeStyle = "red";
		ctx.lineWidth = 2;
		ctx.beginPath();
		const realFrom = game.self.loc.stimes(cfg.tileSize);
		const to = ray.stimes(cfg.tileSize);
		ctx.moveTo(realFrom.x, realFrom.y);
		ctx.lineTo(to.x, to.y);
		ctx.stroke();
	}

	renderMap2D(map) {
		let ctx = this.ctx;

		ctx.clearRect(0, 0, this.width, this.height);
		ctx.strokeStyle = "#000000";
		ctx.lineWidth = 1;
		for (const row of map.tiles) {
			for (const t of row) {
				ctx.fillStyle = cfg.tileColors[t.type];
				ctx.fillRect(t.location.x * cfg.tileSize, t.location.y * cfg.tileSize, cfg.tileSize, cfg.tileSize);
				ctx.strokeRect(t.location.x * cfg.tileSize, t.location.y * cfg.tileSize, cfg.tileSize, cfg.tileSize);
			}
		}
	}

	renderSelf2D(self) {
		let ctx = this.ctx;

		const absLoc = self.loc.stimes(cfg.tileSize);

		ctx.fillStyle = "#46DA2A";
		ctx.beginPath();
		ctx.arc(absLoc.x, absLoc.y, 10, 0, Math.PI*2);
		ctx.fill();

		const dirTo = absLoc.plus(self.dir.stimes(cfg.tileSize));
		ctx.strokeStyle = "#46DA2A";
		ctx.lineWidth = 4;
		ctx.beginPath();
		ctx.moveTo(absLoc.x, absLoc.y);
		ctx.lineTo(dirTo.x, dirTo.y);
		ctx.stroke();
	}
}
