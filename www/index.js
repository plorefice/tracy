import * as wasm from "trtc";

const width = 900;
const height = 550;

var collapsible = document.getElementById('ch02')

// When the 'show' event is triggered, compute and set the canvas size to avoid glitches later on
collapsible.addEventListener('show.bs.collapse', function () {
        var canvas = document.getElementById('canvas');

        /* Workaround for retina displays */
        const pixelRatio = window.devicePixelRatio || 1;
        canvas.width = width;
        canvas.height = height;
        canvas.style.width = `${width / pixelRatio}px`;
        canvas.style.height = `${height / pixelRatio}px`;
})

// When the collapsible is fully shown, start rendering the scene
collapsible.addEventListener('shown.bs.collapse', function () {
        var canvas = document.getElementById('canvas');
        var ctx = canvas.getContext('2d');

        wasm.draw(ctx, width, height);
})
