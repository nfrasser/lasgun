const CopyWebpackPlugin = require("copy-webpack-plugin")
const path = require('path')

const browserConfig = {
  entry: './app.js',
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "app.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],
}

const workerConfig = {
  entry: "./worker.js",
  target: 'webworker',
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "worker.js"
  },
  mode: "development",
}

module.exports = [browserConfig, workerConfig]
