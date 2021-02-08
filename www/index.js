import * as wasm from "trtc";

const width = 900;
const height = 550;

var canvas = document.getElementById('canvas');
var ctx = canvas.getContext('2d');

/* Workaround for retina displays */
const pixelRatio = window.devicePixelRatio || 1;
canvas.width = width;
canvas.height = height;
canvas.style.width = `${width / pixelRatio}px`;
canvas.style.height = `${height / pixelRatio}px`;

wasm.draw(ctx, width, height);