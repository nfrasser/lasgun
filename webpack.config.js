const HtmlWebpackPlugin = require("html-webpack-plugin")
const CopyWebpackPlugin = require("copy-webpack-plugin")
const TerserJSPlugin = require('terser-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const OptimizeCSSAssetsPlugin = require('optimize-css-assets-webpack-plugin');
const path = require('path')

const mode = process.env.NODE_ENV || 'development'
const opt = mode === 'development'
  ? {}
  : {
    optimization: {
      minimizer: [new TerserJSPlugin({}), new OptimizeCSSAssetsPlugin({})],
    }
  }

const resolve = {
  extensions: [ '.tsx', '.ts', '.js', '.wasm']
}

const tsRule = {
  test: /\.tsx?$/,
  use: 'ts-loader',
  exclude: /node_modules/
}

const browserConfig = {
  ...opt,
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
  mode,
  plugins: [
    new CopyWebpackPlugin([
      { from: './www/scenes/*.js', to: './scenes', flatten: true },
      { from: './meshes/*.obj', to: './meshes', flatten: true }
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
  ...opt,
  entry: "./www/worker.ts",
  devtool: 'inline-source-map',
  resolve, module: { rules: [tsRule] },
  target: 'webworker',
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "worker.js"
  },
  mode
}

module.exports = [browserConfig, workerConfig]
