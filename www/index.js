import * as wasm from "trtc";

wasm.init();

const collapsibles = document.getElementsByClassName('accordion-collapse')

Array.from(collapsibles).forEach(collapsible => {
    var canvas = collapsible.getElementsByTagName('canvas')[0]
    var ctx = canvas.getContext('2d')

    // When the 'show' event is triggered, compute and set the canvas size to avoid glitches
    collapsible.addEventListener('show.bs.collapse', function () {
        const { width, height } = wasm.getCanvasSize(collapsible.id);

        /* Workaround for retina displays */
        const pixelRatio = window.devicePixelRatio || 1;
        canvas.width = width;
        canvas.height = height;
        canvas.style.width = `${width / pixelRatio}px`;
        canvas.style.height = `${height / pixelRatio}px`;
    })

    // When the collapsible is fully shown, start rendering the scene
    collapsible.addEventListener('shown.bs.collapse', function () {
        wasm.draw(ctx, collapsible.id);
    })
});
