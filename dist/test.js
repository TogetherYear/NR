const p = require('./NR.win32-x64-msvc.node')

const c = p.getCurrentPositionColor()

console.log(c)

setTimeout(() => {

    p.setMousePosition(100, 200)

    setTimeout(() => {

        p.setButtonClick(p.MosueButton.Left, 100)

        setTimeout(() => {

            p.writeText("TSingleton")

        }, 2000);

    }, 2000);

}, 2000);


