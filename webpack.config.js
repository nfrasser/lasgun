const HtmlWebpackPlugin = require("html-webpack-plugin")
const CopyWebpackPlugin = require("copy-webpack-plugin")
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const path = require('path')

const resolve = {
  extensions: [ '.tsx', '.ts', '.js', '.wasm']
}

const tsRule = {
  test: /\.tsx?$/,
  use: 'ts-loader',
  exclude: /node_modules/
}

const browserConfig = {
  entry: './www/app.ts',
  devtool: 'inline-source-map',
  resolve, module: {
    rules: [
      tsRule,
      {
        test: /\.scss$/,
        use: [
          // fallback to style-loader in development
          process.env.NODE_ENV !== 'production'
            ? 'style-loader'
            : MiniCssExtractPlugin.loader,
          "css-loader",
          "sass-loader"
        ]
      }
    ]
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "app.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin([
      { from: './www/scenes/*.txt', to: './scenes', flatten: true }
    ], {}),
    new HtmlWebpackPlugin({
      template: './www/index.html',
      title: 'lasgun web renderer'
    }),
    new MiniCssExtractPlugin({
      // Options similar to the same options in webpackOptions.output
      // both options are optional
      filename: "[name].css",
      chunkFilename: "[id].css"
    })
  ],
}

const workerConfig = {
  entry: "./www/worker.ts",
  devtool: 'inline-source-map',
  resolve, module: { rules: [tsRule] },
  target: 'webworker',
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "worker.js"
  },
  mode: "development",
}

module.exports = [browserConfig, workerConfig]
