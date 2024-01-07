"use strict";

const LANGUAGE_INDICATORS = [...document.querySelectorAll(".lang")];
const LAYOUT_INDICATORS = [...document.querySelectorAll(".input")];
const MODE_INDICATORS = [...document.querySelectorAll(".typing")];
const SUGGESTIONS = [...document.querySelectorAll("span.suggestion")];

const KEYS = [...document.querySelectorAll("section.keypad > *")];
const KEY_LAYOUTS = [
	["1", "2", "3", "4", "5", "6", "7", "8", "9", "*", "0", "#"],
	["7", "8", "9", "4", "5", "6", "1", "2", "3", ".", "0", "↵"],
	["1", "2", "3", "q", "w", "e", "a", "s", "d", "z", "x", "c"],
];

const MODES = ["MT", "T9"];
const LANGUAGES = ["EN", "PL"];
const LAYOUTS = {
	T9: {
		1: "1",
		2: "2",
		3: "3",
		4: "4",
		5: "5",
		6: "6",
		7: "7",
		8: "8",
		9: "9",
		"*": "*",
		0: "0",
		"#": "#",
	},
	NUM: {
		7: "1",
		8: "2",
		9: "3",
		4: "4",
		5: "5",
		6: "6",
		1: "7",
		2: "8",
		3: "9",
		",": "*",
		".": "*",
		0: "0",
		"↵": "#",
		Enter: "#",
	},
	KBD: {
		1: "1",
		2: "2",
		3: "3",
		q: "4",
		Q: "4",
		w: "5",
		W: "5",
		e: "6",
		E: "6",
		a: "7",
		A: "7",
		s: "8",
		S: "8",
		d: "9",
		D: "9",
		z: "*",
		Z: "*",
		x: "0",
		X: "0",
		c: "#",
		C: "#",
	},
};

const AUDIO = {
	1: { low: 697, high: 1209 },
	2: { low: 697, high: 1336 },
	3: { low: 697, high: 1477 },
	4: { low: 770, high: 1209 },
	5: { low: 770, high: 1336 },
	6: { low: 770, high: 1477 },
	7: { low: 852, high: 1209 },
	8: { low: 852, high: 1336 },
	9: { low: 852, high: 1477 },
	"*": { low: 941, high: 1209 },
	0: { low: 941, high: 1336 },
	"#": { low: 941, high: 1477 },
};

let selected = 0;
let language = 0;
let layout = 0;
let mode = 0;

window.nextLanguage = () => {
	LANGUAGE_INDICATORS[language].dataset.selected = false;
	language += 1;
	language %= LANGUAGE_INDICATORS.length;
	LANGUAGE_INDICATORS[language].dataset.selected = true;
	console.debug(`lang:${LANGUAGES[language]}`);
	window.ipc.postMessage(`lang:${LANGUAGES[language]}`);
};

window.nextLayout = () => {
	LAYOUT_INDICATORS[layout].dataset.selected = false;
	layout += 1;
	layout %= LAYOUT_INDICATORS.length;
	LAYOUT_INDICATORS[layout].dataset.selected = true;
	KEYS.forEach((e, i) => (e.firstChild.textContent = KEY_LAYOUTS[layout][i]));
};

window.nextMode = () => {
	MODE_INDICATORS[mode].dataset.selected = false;
	mode += 1;
	mode %= MODE_INDICATORS.length;
	MODE_INDICATORS[mode].dataset.selected = true;
	console.debug(`mode:${MODES[mode]}`);
	window.ipc.postMessage(`mode:${MODES[mode]}`);
};

window.text = "";
window.suggestions = ["", ":-)", ":-("];

window.refresh = () => {
	const displayText = document.querySelector("output.text");
	const suggestions = document.querySelectorAll("span.suggestion");

	displayText.dataset.initial = false;
	displayText.textContent = window.text;
	suggestions.forEach((e, i) => (e.textContent = window.suggestions[i]));
};

const input = (inputKey) => {
	const key = LAYOUTS[Object.keys(LAYOUTS)[layout]][inputKey];

	try {
		const ctx = window.dtmfAudioContext || new AudioContext();
		window.dtmfAudioContext = ctx;
		const gain = new GainNode(ctx, { gain: 0.1 });

		const low = new OscillatorNode(ctx, {
			frequency: AUDIO[key].low,
			type: "sine",
		});
		const high = new OscillatorNode(ctx, {
			frequency: AUDIO[key].high,
			type: "sine",
		});

		low.connect(gain).connect(ctx.destination);
		high.connect(gain).connect(ctx.destination);

		const soon = ctx.currentTime + 0.001;

		low.start(soon);
		high.start(soon);

		low.stop(soon + 0.3);
		high.stop(soon + 0.3);
	} catch (e) {
		console.warn(e);
		window.dtmfAudioContext = undefined;
	}

	if (key === "*") {
		SUGGESTIONS[selected].dataset.selected = false;
		selected += 1;
		selected %= SUGGESTIONS.length;
		SUGGESTIONS[selected].dataset.selected = true;
	} else if (key === "0" && window.suggestions[selected] != "") {
		SUGGESTIONS[selected].dataset.selected = false;
		window.text += SUGGESTIONS[selected].textContent + " ";
		window.suggestions = ["", "", ""];
		selected = 0;
		SUGGESTIONS[selected].dataset.selected = true;
	} else if (key === "#" && window.suggestions[0] === "") {
		const words = window.text.split(" ");
		window.text = words.splice(0, words.length - 2).join(" ") + " ";
		refresh();
	}

	if (key !== undefined) {
		console.debug(`input:${key}`);
		window.ipc.postMessage(`input:${key}`);
	}
};

[...document.querySelectorAll("button.key")].forEach((element) =>
	element.addEventListener("click", (e) =>
		input(e.currentTarget.firstChild.textContent)
	)
);

document.addEventListener("keydown", (e) => input(e.key));
document.addEventListener("keydown", (e) => {
	if (e.key === "Escape") {
		window.refresh();
	}
});
