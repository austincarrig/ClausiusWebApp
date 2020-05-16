const InnerRadius = 6.5;
const SmallOuterRadius = 9.0;
const SmallOuterLineWidth = 0.75;
const LargeOuterRadius = 60.0;
const LargeOuterLineWidth = 5.0;
const HitmarkerLength = 25.0;
const HitmarkerLineWidth = 1.0;

const orangeColor = "#ffa500"

var canvas, _ctx, flag = false,
    prevX = 0,
    currX = 0,
    prevY = 0,
    currY = 0;

function initCanvas() {
    canvas.width = document.querySelector("#canvas-border").scrollWidth
    canvas.height = document.querySelector("#canvas-border").scrollHeight
}

function initialization() {
    canvas = document.querySelector("#canvas")

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

    document.querySelector("#canvas").addEventListener("load", initCanvas)
    document.querySelector("body").addEventListener("resize", initCanvas)
}

function findxy(res, event) {
    initCanvas();
    if (res == 'down') {
        prevX = currX;
        prevY = currY;
        currX = event.clientX - canvas.offsetLeft;
        currY = event.clientY - canvas.offsetTop;

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
            prevX = currX;
            prevY = currY;
            currX = event.clientX - canvas.offsetLeft;
            currY = event.clientY - canvas.offsetTop;
            DrawLargeIndicator();
        }
    }
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

function DrawLargeIndicator() {
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