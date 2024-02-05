const p = require('./NR.win32-x64-msvc.node')

setTimeout(() => {

    p.SetMousePosition(100, 200)

    setTimeout(() => {

        p.SetButtonClick(p.MosueButton.Left, 100)

        setTimeout(() => {

            p.WriteText("TSingleton")

        }, 2000);

    }, 2000);

}, 2000);


