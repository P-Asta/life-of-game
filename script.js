import init, { LifeOfGame } from './pkg/wasm_pack_pra.js'
await init()
let game = LifeOfGame.new(30);
game.draw()
draw();
let PUSE = 1;

function toggle(i, j) {
    game.toggle(i, j);
}
function draw() {
    game.draw();
    document.querySelectorAll("div.tile").forEach((button) => {
        if (button.id.indexOf("-") != -1) {
            button.addEventListener("click", () => {
                let [i, j] = button.id.split("-").map(Number);
                toggle(i, j);
                draw();
            });
        }
    })
}

document.getElementById("puse").addEventListener("click", () => {
    PUSE = !PUSE;
    let e = document.getElementById("puse");
    if (PUSE) {
        e.value = "START"
    } else {
        e.value = "PUSE"
    }
})
function loop() {
    if (!PUSE) {
        game.step();
        draw();
    }
    setTimeout(() => {
        requestAnimationFrame(loop);
    }, 100);
}
loop();