import init, { LifeOfGame } from './pkg/wasm_pack_pra.js'
await init()
alert("move: arrow key\npuse: space");
let game = LifeOfGame.new();

draw();
let PUSE = 1;
let SPEED = 100;

function toggle(i, j) {
    game.toggle(i, j);
}
function draw() {
    game.draw(window.innerWidth / 20, window.innerHeight / 20);
    document.querySelectorAll("div.tile").forEach((button) => {
        if (button.id.indexOf(",") != -1) {
            button.addEventListener('click', () => {
                let [i, j] = button.id.split(",").map(Number);
                toggle(i, j);
                draw();
            });
        }
    })
}


// document.getElementById("puse").addEventListener("click", () => {
//     PUSE = !PUSE;
//     let e = document.getElementById("puse");
//     if (PUSE) {
//         e.value = "START"
//     } else {
//         e.value = "PUSE"
//     }
// })

window.onkeydown = (e) => {
    if (e.key == "ArrowRight") {
        game.move_camera(1, 0);
        draw();
    } if (e.key == "ArrowLeft") {
        game.move_camera(-1, 0);
        draw();
    } if (e.key == "ArrowDown") {
        game.move_camera(0, 1);
        draw();
    }
    if (e.key == "ArrowUp") {
        game.move_camera(0, -1);
        draw();
    }
    if (e.key == " ") {
        PUSE = !PUSE;
    }
    console.log(e.key)
}

setInterval(() => {
    if (!PUSE) {
        console.log("step")
        game.step();
        draw();
    }
}, 100);

// const zoomElement = document.querySelector("main");
// let zoom = 1;
// const ZOOM_SPEED = 0.1;

// document.addEventListener("wheel", function (e) {
//     if (zoom >= 2000) {
//         zoom = 2000;
//     }
//     if (zoom <= 0) {
//         zoom = 0;
//     }
//     if (e.deltaY > 0) {
//         zoomElement.style.transform = `scale(${(zoom -= ZOOM_SPEED)})`;
//     } else {
//         zoomElement.style.transform = `scale(${(zoom += ZOOM_SPEED)})`;
//     }
// });