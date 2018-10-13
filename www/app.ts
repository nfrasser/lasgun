// @ts-ignore
import './app.scss'
import * as CodeMirror from 'codemirror'

async function main() {
    const canvas = document.getElementById('output') as HTMLCanvasElement
    const ctx = canvas.getContext('2d')
    const renderButton = document.getElementById('render-button')

    let worker = new Worker('./worker.js')
    worker.addEventListener('message', function (event) {
        let imageData: ImageData

        switch (event.data.type) {
        case 'status':
            if (event.data.value !== 'ready') break
            renderButton.removeAttribute('disabled')
            break
        case 'scene':
            let { width, height } = event.data.value
            canvas.width = width
            canvas.height = height
            break
        case 'hunk':
            let {x, y, data} = event.data.value
            imageData = new ImageData(data, 16, 16)
            requestAnimationFrame(() => ctx.putImageData(imageData, x, y))
            break
        case 'done':
            let { start, end } = event.data.value
            console.log(`Render time: ${end - start}ms (${(end - start)/1000}) sec`)
        }
    })

    let response = await fetch('./scenes/simplecows.js')
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
