const InnerRadius = 6.5;
const SmallOuterRadius = 9.0;
const SmallOuterLineWidth = 0.75;
const LargeOuterRadius = 60.0;
const LargeOuterLineWidth = 5.0;
const HitmarkerLength = 25.0;
const HitmarkerLineWidth = 1.0;

const orangeColor = "#ffa500"

var canvas, canvasBorder, _ctx, flag = false,
    prevX = 0,
    currX = 0,
    prevY = 0,
    currY = 0;

function initialization() {
    canvas = document.querySelector("#canvas")
    canvasBorder = document.querySelector("#canvas-border")

    _ctx = canvas.getContext("2d");

    canvas.addEventListener("mousemove", function (event) {
        findxy('move', event)
    }, false);
    canvas.addEventListener("mousedown", function (event) {
        findxy('down', event)
    }, false);
    canvas.addEventListener("mouseup", function (event) {
        findxy('up', event)
    }, false);
    canvas.addEventListener("mouseout", function (event) {
        findxy('out', event)
    }, false);

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

function findxy(res, event) {
    if (res == 'down') {
        resizeCanvas();

        flag = true;
        
        DrawLargeIndicator();
    }
    if (res == 'up' || res == "out") {
        if (flag) {
            flag = false;

            DrawSmallIndicator();
        }
    }
    if (res == 'move') {
        if (flag) {
            DrawLargeIndicator();
        }
    }
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