import init, { LifeOfGame } from './pkg/wasm_pack_pra.js'
await init()
let game = LifeOfGame.new(50);
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
            button.addEventListener('click', () => {
                let [i, j] = button.id.split("-").map(Number);
                toggle(i, j);
                draw();
            });
        }
    })
}

document.getElementById("resize").addEventListener("click", () => {
    game = LifeOfGame.new(Number(prompt("Enter the size of the grid", 50)));
    game.draw();
    draw()
    PUSE = 1
    let e = document.getElementById("puse");
    if (PUSE) {
        e.value = "START"
    } else {
        e.value = "PUSE"
    }
})

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
    requestAnimationFrame(loop);
}
loop();