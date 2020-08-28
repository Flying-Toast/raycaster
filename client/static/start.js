import init from "./client.js";

addEventListener("load", function() {
	main();
});

function main() {
	(async function() {
		await init();
	})();
}
