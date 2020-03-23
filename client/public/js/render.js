import { cfg } from "./config.js";

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
		let ctx = this.ctx;

		ctx.clearRect(0, 0, this.width, this.height);
		for (const row of game.map.tiles) {
			for (const t of row) {
				ctx.fillStyle = cfg.tileColors[t.type];
				ctx.fillRect(t.location.x * cfg.tileSize, t.location.y * cfg.tileSize, cfg.tileSize, cfg.tileSize);
			}
		}

		this.renderSelf2D(game.self);
	}

	renderSelf2D(self) {
		let ctx = this.ctx;

		const absLoc = self.loc.stimes(cfg.tileSize);

		ctx.fillStyle = "#46DA2A";
		ctx.beginPath();
		ctx.arc(absLoc.x, absLoc.y, 5, 0, Math.PI*2);
		ctx.fill();

		const dirTo = absLoc.plus(self.dir.stimes(cfg.tileSize));
		ctx.strokeStyle = "#46DA2A";
		ctx.lineWidth = 2;
		ctx.beginPath();
		ctx.moveTo(absLoc.x, absLoc.y);
		ctx.lineTo(dirTo.x, dirTo.y);
		ctx.stroke();
	}
}
