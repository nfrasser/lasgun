var worker = new Worker('./worker.js')
worker.addEventListener('message', function (event) {
    const canvas = document.getElementById('output')
    const ctx = canvas.getContext('2d')
    const imageData = new ImageData(event.data, 512, 512)
    ctx.putImageData(imageData, 0, 0)
})
