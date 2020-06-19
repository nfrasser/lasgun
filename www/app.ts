// @ts-ignore
import './app.scss'
import * as CodeMirror from 'codemirror'
import 'codemirror/mode/javascript/javascript'

async function main() {
    const canvas = document.getElementById('output') as HTMLCanvasElement
    const ctx = canvas.getContext('2d')
    const renderButton = document.getElementById('render-button')
    const cancelButton = document.getElementById('cancel-button')

    const setupWorker = () => {
        const worker = new Worker('./worker.js')
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
                renderButton.removeAttribute('disabled')
                break
            case 'error':
                alert(JSON.stringify(event.data.value))
                renderButton.removeAttribute('disabled')
                break
            }
        })
        return worker
    }

    let worker: Worker = setupWorker()
    let response = await fetch('./scenes/simple.js')
    let scene = await response.text()

    let code = CodeMirror(document.getElementById('code'), {
        value: scene,
        lineNumbers: true,
        theme: 'material-palenight'
    })

    renderButton.addEventListener('click', () => {
        worker.postMessage({ type: 'scene', value: code.getValue() })
        renderButton.setAttribute('disabled', 'disabled')
        window.scrollTo(0, 0)
    })

    cancelButton.addEventListener('click', () => {
        worker.terminate()
        worker = setupWorker()
        renderButton.removeAttribute('disabled')
    })
}

main()
