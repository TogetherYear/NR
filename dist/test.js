const p = require('./NR.win32-x64-msvc.node')

setTimeout(() => {
    p.setMousePosition(100, 100)
}, 5000);