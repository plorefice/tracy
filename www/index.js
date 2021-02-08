import * as wasm from "trtc";

const width = 900;
const height = 550;

var canvas = document.getElementById('canvas');
var ctx = canvas.getContext('2d');

/* Workaround for retina displays */
const pixelRatio = window.devicePixelRatio || 1;

canvas.width = width * pixelRatio;
canvas.height = height * pixelRatio;
canvas.style.width = `${width}px`;
canvas.style.height = `${height}px`;
ctx.scale(pixelRatio, pixelRatio);

wasm.draw(ctx, width, height);