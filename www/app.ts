import { javascript } from '@codemirror/lang-javascript'
import { EditorView, basicSetup } from 'codemirror'
import { EditorState } from '@codemirror/state'
import { dracula } from 'thememirror'

async function main() {
  const canvas = document.getElementById('output') as HTMLCanvasElement
  const ctx = canvas.getContext('2d')
  const renderForm = document.getElementById('render') as HTMLFormElement
  const renderButton = document.getElementById('render-button')
  const cancelButton = document.getElementById('cancel-button')
  let width = 1
  let height = 1

  const setupWorker = () => {
    const worker = new Worker(new URL('./worker.ts', import.meta.url))
    worker.addEventListener('message', function (event) {
      let imageData: ImageData

      switch (event.data.type) {
        case 'status':
          if (event.data.value !== 'ready') break
          renderButton.removeAttribute('disabled')
          break
        case 'progress':
          const { output } = event.data.value
          imageData = new ImageData(new Uint8ClampedArray(output), width, height)
          requestAnimationFrame(() => ctx.putImageData(imageData, 0, 0))
          break
        case 'done':
          const { start, end } = event.data.value
          console.log(`Render time: ${end - start}ms (${(end - start) / 1000}) sec`)
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
  const response = await fetch('./scenes/simple.js')
  const scene = await response.text()

  const code = new EditorView({
    state: EditorState.create({
      doc: scene,
      extensions: [basicSetup, dracula, javascript()]
    }),
    parent: document.getElementById('code')
  })

  renderForm.addEventListener('submit', (event) => {
    event.preventDefault()
    if (renderButton.hasAttribute('disabled') || !renderForm.checkValidity()) return

    const data = new FormData(renderForm)
    width = parseInt(data.get('width') as string, 10)
    height = parseInt(data.get('height') as string, 10)
    canvas.width = width
    canvas.height = height

    worker.postMessage({
      type: 'scene',
      value: JSON.stringify({ scene: code.state.doc.toString(), width, height })
    })

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
