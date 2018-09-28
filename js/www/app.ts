// @ts-ignore
import './app.scss'
import * as CodeMirror from 'codemirror'

async function main() {
    const canvas = document.getElementById('output') as HTMLCanvasElement
    const ctx = canvas.getContext('2d')
    const renderButton = document.getElementById('render-button')

    let worker = new Worker('./worker.js')
    worker.addEventListener('message', function (event) {
        let imageData

        switch (event.data.type) {
        case 'status':
            if (event.data.value !== 'ready') break
            renderButton.removeAttribute('disabled')
            break
        case 'image':
            imageData = new ImageData(event.data.value, 512, 512)
            ctx.putImageData(imageData, 0, 0)
            break
        }
    })

    let response = await fetch('./scenes/simple.js.txt')
    let scene = await response.text()

    let code = CodeMirror(document.body, {
        value: scene,
        lineNumbers: true,
        theme: 'solarized'
    })

    renderButton.addEventListener('click', () => {
        worker.postMessage({ type: 'scene', value: code.getValue() })
    })

}

main()
