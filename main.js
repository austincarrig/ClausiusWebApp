import wasmInit from "./pkg/exports.js";
import * as wasm from "./pkg/exports.js";

const runWasm = async () => {
  // Instantiate our wasm module
  const rustWasm = await wasmInit("./pkg/exports_bg.wasm");

  // Call the Add function export from wasm, save the result
  let result = wasm.call_me_from_javascript();
};
runWasm();

const InnerRadius = 6.5;
const SmallOuterRadius = 9.0;
const SmallOuterLineWidth = 0.75;
const LargeOuterRadius = 60.0;
const LargeOuterLineWidth = 5.0;
const HitmarkerLength = 25.0;
const HitmarkerLineWidth = 1.0;

const orangeColor = "#ffa500"

var canvas, canvasBorder, _ctx, touchPresent = false,
    prevX = 0,
    currX = 0,
    prevY = 0,
    currY = 0;

function initialization() {
    canvas = document.querySelector("#canvas")
    canvasBorder = document.querySelector("#canvas-border")

    _ctx = canvas.getContext("2d");

    // This Event Listener handles all mouse movement within
    // the bounds of the canvas
    // Triggers whether the mouse button is pressed or not
    canvas.addEventListener("mousemove", function (event) {
        TouchRegistered('move', event)
    }, false);

    // Event listener to handle initial mouse click
    canvas.addEventListener("mousedown", function (event) {
        TouchRegistered('down', event)
    }, false);

    // Event listener to handle release of mouse click
    canvas.addEventListener("mouseup", function (event) {
        TouchRegistered('up', event)
    }, false);

    // Event listener to handle mouse movement leaving
    // the bounds of the canvas
    canvas.addEventListener("mouseout", function (event) {
        TouchRegistered('out', event)
    }, false);

    // We want to know if the window changes size so we
    // can resize the drawing canvas accordingly
    window.addEventListener("resize", resizeCanvasBorder)
    window.addEventListener("load", resizeCanvasBorder)
}

function resizeCanvasBorder() {
    const innerWidth = document.querySelector("#canvas-border").scrollWidth;

    canvasBorder.style.maxHeight = (innerWidth * 0.75) + "px";
    canvasBorder.style.maxWidth  = innerWidth + "px";
    resizeCanvas()
}

function resizeCanvas() {
    canvas.width = canvas.scrollWidth
    canvas.height = canvas.scrollHeight
}

function TouchRegistered(res, event) {
    if (res == 'down') {
        resizeCanvas();
        
        DrawLargeIndicator();

        touchPresent = true;
    }
    if (res == 'move') {
        if (touchPresent) {
            DrawLargeIndicator();
        }
    }
    if (res == 'up' || res == "out") {
        if (touchPresent) {
            DrawSmallIndicator();

            touchPresent = false;
        }
    }
}

function newFunc() {
    
    // Call the Add function export from wasm, save the result
    let result = wasm.call_me_from_javascript();

    console.log(result.get_t()); // Should output '72'
}

function DrawLargeIndicator() {
    // Set new values for location
    prevX = currX;
    prevY = currY;

    currX = (event.clientX - canvas.getBoundingClientRect().left);
    currY = (event.clientY - canvas.getBoundingClientRect().top);
    
    // Remove all previously-drawn paths from the canvas
    _ctx.clearRect(0, 0, canvas.width, canvas.height)

    // The large indicator consists of...

    // ... an inner circle...
    DrawInnerCircle(_ctx);

    // ... a large ring...
    DrawRing(_ctx,
             LargeOuterRadius,
             LargeOuterLineWidth);
    
    // ... and 4 hitmarkers.
    DrawHitmarkers(_ctx);
}

function DrawSmallIndicator() {
    // Remove all previously-drawn paths from the canvas
    _ctx.clearRect(0, 0, canvas.width, canvas.height)
    
    // The small indicator consists of...

    // ... an inner circle...
    DrawInnerCircle(_ctx);

    // ... and a small ring.
    DrawRing(_ctx,
             SmallOuterRadius,
             SmallOuterLineWidth);
}

function DrawInnerCircle(ctx) {
    ctx.beginPath()
    ctx.arc(
        currX,
        currY,
        InnerRadius,
        0,
        2 * Math.PI
    )
    ctx.fillStyle = orangeColor
    ctx.fill()
    ctx.closePath()
}

function DrawRing(ctx,
                  radius,
                  width) {
    ctx.beginPath()
    ctx.arc(
        currX,
        currY,
        radius,
        0,
        2 * Math.PI
    )
    ctx.strokeStyle = orangeColor
    ctx.lineWidth = width
    ctx.stroke()
    ctx.closePath()
}

function DrawHitmarkers(ctx)
{
    // Draw Top Hitmarker
    DrawLine(ctx,
             currX,
             currY - LargeOuterRadius,
             currX,
             currY - (LargeOuterRadius - HitmarkerLength));

    // Draw Left Hitmarker
    DrawLine(ctx,
             currX - LargeOuterRadius,
             currY,
             currX - (LargeOuterRadius - HitmarkerLength),
             currY);

    // Draw Bottom Hitmarker
    DrawLine(ctx,
             currX,
             currY + LargeOuterRadius,
             currX,
             currY + (LargeOuterRadius - HitmarkerLength));

    // Draw Right Hitmarker
    DrawLine(ctx,
             currX + LargeOuterRadius,
             currY,
             currX + (LargeOuterRadius - HitmarkerLength),
             currY);
}

function DrawLine(ctx,
                  startX,
                  startY,
                  endX,
                  endY) {
    ctx.beginPath()
    ctx.moveTo(startX, startY)
    ctx.lineTo(endX, endY)
    ctx.strokeStyle = orangeColor
    ctx.lineWidth = HitmarkerLineWidth
    ctx.stroke()
    ctx.closePath()
}

initialization();