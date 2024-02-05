const path = require('path')

const p = require('./NR.win32-x64-msvc.node')

const c = p.GetCurrentPositionColor()

console.log(c)

setTimeout(() => {

    p.SetMousePosition(100, 200)

    setTimeout(() => {

        p.SetButtonClick(p.MosueButton.Left, 100)

        setTimeout(() => {

            p.WriteText("TSingleton")

        }, 2000);

    }, 2000);

}, 2000);


