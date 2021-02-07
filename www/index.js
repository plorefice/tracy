import * as wasm from "trtc";

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

wasm.draw(ctx, canvas.width, canvas.height);