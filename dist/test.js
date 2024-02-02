const p = require('./NR.win32-x64-msvc.node')

setTimeout(() => {

    p.setMousePosition(100, 100)

    setTimeout(() => {

        p.setButtonClick(p.MosueButton.Right, 100)

    }, 2000);

}, 2000);


